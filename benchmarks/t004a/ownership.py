"""Read-only macOS ownership observer; private ABI, benchmark-only, fail closed.

Source of responsibility_get_pid_responsible_for_pid(pid_t) -> pid_t:
https://chromium.googlesource.com/chromium/src/+/refs/tags/134.0.6998.28/base/process/process_info_mac.mm
Symbol also verified in local macOS SDK libSystem.B.tbd / libquarantine.tbd.
Coalition ABI from Apple xnu commit f6217f891ac0bb64f3d375211650a4c1ff8ca1ea:
  bsd/sys/proc_info_private.h: struct proc_pidcoalitioninfo; flavor 20
  osfmk/mach/coalition.h: RESOURCE=0, JETSAM=1, NUM_TYPES=2
https://github.com/apple-oss-distributions/xnu/tree/f6217f891ac0bb64f3d375211650a4c1ff8ca1ea

Coalition identity is corroborating evidence ONLY: shell-launched GUI apps can
share both responsibility and coalitions with the launcher and sibling apps.
No private setters, process signals, privilege elevation, or app changes.
"""

import argparse
import ctypes
import json
import os
import platform
import subprocess
import time

PROC_PIDCOALITIONINFO = 20
COALITION_FIELD_COUNT = 5


def classify(main_pid, facts):
    """Return only PIDs linked directly to this app, never a shared launcher."""
    by_pid = {row["pid"]: row for row in facts}
    main = by_pid.get(main_pid)
    if main is None:
        return {"pids": [], "self_responsible": False, "links": {}, "error": "main PID absent"}
    links = {main_pid: "main process"}
    for row in facts:
        if row["pid"] != main_pid and row.get("responsible_pid") == main_pid:
            links[row["pid"]] = "responsibility API equals main PID"
    changed = True
    while changed:
        changed = False
        for row in facts:
            if row["pid"] not in links and row["ppid"] in links:
                links[row["pid"]] = f"parent chain via PID {row['ppid']}"
                changed = True
    return {
        "pids": sorted(links),
        "self_responsible": main.get("responsible_pid") == main_pid,
        "links": links,
        "error": None,
    }


def _native_functions():
    system = ctypes.CDLL("/usr/lib/libSystem.B.dylib", use_errno=True)
    responsible = system.responsibility_get_pid_responsible_for_pid
    responsible.argtypes = [ctypes.c_int]
    responsible.restype = ctypes.c_int
    libproc = ctypes.CDLL("/usr/lib/libproc.dylib", use_errno=True)
    pidinfo = libproc.proc_pidinfo
    pidinfo.argtypes = [
        ctypes.c_int, ctypes.c_int, ctypes.c_uint64, ctypes.c_void_p, ctypes.c_int
    ]
    pidinfo.restype = ctypes.c_int
    return responsible, pidinfo


def _coalitions(pid, pidinfo):
    fields = (ctypes.c_uint64 * COALITION_FIELD_COUNT)()
    ctypes.set_errno(0)
    size = pidinfo(pid, PROC_PIDCOALITIONINFO, 0, fields, ctypes.sizeof(fields))
    error_number = ctypes.get_errno()
    valid = size == ctypes.sizeof(fields)
    return {
        "resource_coalition": int(fields[0]) if valid else None,
        "jetsam_coalition": int(fields[1]) if valid else None,
        "coalition_return_size": size,
        "coalition_errno": error_number,
    }


def assign(main_pid):
    """Observe process ownership. Caller must still establish workload readiness.

    `pids` is a defensible attributed subset. `self_responsible` does NOT certify
    completeness: helpers may start/restart later; rerun during measurement.
    Shell launch often gives only main+direct children: mark total unresolved.
    """
    if platform.system() != "Darwin":
        raise RuntimeError("ownership observer requires macOS")
    if not isinstance(main_pid, int) or main_pid <= 0:
        raise ValueError("main_pid must be a positive integer")
    responsible, pidinfo = _native_functions()
    output = subprocess.check_output(
        ["ps", "-axo", "pid=,ppid=,comm="], text=True, timeout=10
    )
    facts = []
    for line in output.splitlines():
        columns = line.strip().split(None, 2)
        if len(columns) != 3:
            continue
        pid, ppid = int(columns[0]), int(columns[1])
        ctypes.set_errno(0)
        owner = responsible(pid)
        facts.append({
            "pid": pid,
            "ppid": ppid,
            "name": columns[2],
            "responsible_pid": owner if owner > 0 else None,
            "responsibility_errno": ctypes.get_errno(),
        })
    result = classify(main_pid, facts)
    relevant = [
        row for row in facts
        if row["pid"] in result["pids"]
        or row["pid"] == os.getpid()
        or "WebKit" in row["name"]
    ]
    for row in relevant:
        row.update(_coalitions(row["pid"], pidinfo))
    return {
        "captured_unix_seconds": time.time(),
        "main_pid": main_pid,
        "observer_pid": os.getpid(),
        "method": "private read-only macOS responsibility API; parent chain; coalition corroboration",
        **result,
        "evidence": relevant,
        "limitation": "Attributed subset, not proof of all helpers. Shared launcher coalitions never imply app ownership.",
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("pid", type=int)
    args = parser.parse_args()
    print(json.dumps(assign(args.pid), indent=2))


if __name__ == "__main__":
    main()
