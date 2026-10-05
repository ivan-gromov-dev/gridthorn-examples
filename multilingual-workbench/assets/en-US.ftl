welcome = Welcome, { $name }!
workers = { $count ->
    [one] One worker
   *[other] { $count } workers
}
role = { $role ->
    [worker] Worker
   *[other] Visitor
}
balance = Balance: { NUMBER($amount, minimumFractionDigits: 2) }
fallback-only = This message comes from the English fallback catalog.
review = Open review dialog
animation = Animate windows
nested = Open nested confirmation
context = Context actions
close = Close review
confirm = Confirm and return
sample = Insert multilingual sample
close-context = Close context
editor-hint = Edit text above. Drag the slider to change the worker count.
    Tab / Shift+Tab: focus · Enter: activate · Escape: close top window
    Ctrl/Command+A/C/X/V: select, copy, cut and paste · Wheel: scroll
