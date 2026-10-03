welcome = Добро пожаловать, { $name }!
workers = { $count ->
    [one] { $count } работник
    [few] { $count } работника
    [many] { $count } работников
   *[other] { $count } работника
}
role = { $role ->
    [worker] Работник
   *[other] Посетитель
}
balance = Баланс: { NUMBER($amount, minimumFractionDigits: 2) }
