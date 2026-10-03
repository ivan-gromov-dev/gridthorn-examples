welcome = مرحبًا، { $name }!
workers = { $count ->
    [zero] لا عمال
    [one] عامل واحد
    [two] عاملان
    [few] { $count } عمال
    [many] { $count } عاملًا
   *[other] { $count } عامل
}
role = { $role ->
    [worker] عامل
   *[other] زائر
}
balance = الرصيد: { NUMBER($amount, minimumFractionDigits: 2) }
