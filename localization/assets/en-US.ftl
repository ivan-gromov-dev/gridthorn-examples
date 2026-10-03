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
