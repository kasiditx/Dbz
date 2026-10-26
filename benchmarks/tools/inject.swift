// Real OS input injector (CGEvent, HID tap) for T-003 follow-up E2E checks.
// Needs Accessibility permission for the invoking process. Coordinates: global screen points.
// Usage: inject key <vkcode> [cmd,shift,alt,ctrl] | type <text> | click x y [n] | move x y
//        inject drag x1 y1 x2 y2 [steps] | scroll x y dy | now
import Foundation
import CoreGraphics

func flags(_ s: String?) -> CGEventFlags {
    var f: CGEventFlags = []
    for m in (s ?? "").split(separator: ",") {
        switch m { case "cmd": f.insert(.maskCommand); case "shift": f.insert(.maskShift)
                   case "alt": f.insert(.maskAlternate); case "ctrl": f.insert(.maskControl); default: break }
    }
    return f
}
func post(_ e: CGEvent?) { e?.post(tap: .cghidEventTap) }
func nowNs() -> UInt64 { var ts = timespec(); clock_gettime(CLOCK_REALTIME, &ts); return UInt64(ts.tv_sec) * 1_000_000_000 + UInt64(ts.tv_nsec) }
func sleepMs(_ ms: Int) { usleep(UInt32(ms * 1000)) }
let a = CommandLine.arguments
guard a.count >= 2 else { print("usage"); exit(2) }
switch a[1] {
case "now": print(nowNs())
case "key":
    let code = CGKeyCode(UInt16(a[2])!)
    let f = flags(a.count > 3 ? a[3] : nil)
    let src = CGEventSource(stateID: .hidSystemState)
    let d = CGEvent(keyboardEventSource: src, virtualKey: code, keyDown: true); d?.flags = f
    let u = CGEvent(keyboardEventSource: src, virtualKey: code, keyDown: false); u?.flags = f
    print(nowNs()); post(d); sleepMs(40); post(u)
case "type":
    var printed = false
    for ch in a[2] {
        let u16 = Array(String(ch).utf16)
        let src = CGEventSource(stateID: .hidSystemState)
        let d = CGEvent(keyboardEventSource: src, virtualKey: 0, keyDown: true)
        d?.flags = []   // do not inherit modifier flags left by a previous `key cmd,...`
        d?.keyboardSetUnicodeString(stringLength: u16.count, unicodeString: u16)
        let u = CGEvent(keyboardEventSource: src, virtualKey: 0, keyDown: false)
        u?.flags = []
        u?.keyboardSetUnicodeString(stringLength: u16.count, unicodeString: u16)
        if !printed { print(nowNs()); printed = true }   // timestamp right before first post
        post(d); sleepMs(25); post(u); sleepMs(25)
    }
case "move": post(CGEvent(mouseEventSource: nil, mouseType: .mouseMoved, mouseCursorPosition: CGPoint(x: Double(a[2])!, y: Double(a[3])!), mouseButton: .left))
case "click":
    let p = CGPoint(x: Double(a[2])!, y: Double(a[3])!)
    let n = a.count > 4 ? Int(a[4])! : 1
    post(CGEvent(mouseEventSource: nil, mouseType: .mouseMoved, mouseCursorPosition: p, mouseButton: .left)); sleepMs(50)
    for i in 1...n {
        let d = CGEvent(mouseEventSource: nil, mouseType: .leftMouseDown, mouseCursorPosition: p, mouseButton: .left)
        d?.setIntegerValueField(.mouseEventClickState, value: Int64(i))
        let u = CGEvent(mouseEventSource: nil, mouseType: .leftMouseUp, mouseCursorPosition: p, mouseButton: .left)
        u?.setIntegerValueField(.mouseEventClickState, value: Int64(i))
        print(nowNs()); post(d); post(u); sleepMs(40)
    }
case "drag":
    let p1 = CGPoint(x: Double(a[2])!, y: Double(a[3])!), p2 = CGPoint(x: Double(a[4])!, y: Double(a[5])!)
    let steps = a.count > 6 ? Int(a[6])! : 20
    post(CGEvent(mouseEventSource: nil, mouseType: .mouseMoved, mouseCursorPosition: p1, mouseButton: .left)); sleepMs(80)
    post(CGEvent(mouseEventSource: nil, mouseType: .leftMouseDown, mouseCursorPosition: p1, mouseButton: .left)); sleepMs(80)
    for i in 1...steps {
        let t = Double(i) / Double(steps)
        let p = CGPoint(x: p1.x + (p2.x - p1.x) * t, y: p1.y + (p2.y - p1.y) * t)
        post(CGEvent(mouseEventSource: nil, mouseType: .leftMouseDragged, mouseCursorPosition: p, mouseButton: .left)); sleepMs(16)
    }
    post(CGEvent(mouseEventSource: nil, mouseType: .leftMouseUp, mouseCursorPosition: p2, mouseButton: .left))
case "scroll":
    let p = CGPoint(x: Double(a[2])!, y: Double(a[3])!)
    post(CGEvent(mouseEventSource: nil, mouseType: .mouseMoved, mouseCursorPosition: p, mouseButton: .left)); sleepMs(50)
    print(nowNs())
    let unit: CGScrollEventUnit = (a.count > 5 && a[5] == "pixel") ? .pixel : .line
    let src = CGEventSource(stateID: .hidSystemState)
    for _ in 0..<5 {   // several notches: a single event can be coalesced/ignored
        let e = CGEvent(scrollWheelEvent2Source: src, units: unit, wheelCount: 1, wheel1: Int32(a[4])!, wheel2: 0, wheel3: 0)
        e?.flags = []
        post(e); sleepMs(30)
    }
default: print("unknown"); exit(2)
}
