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
review = Открыть проверку
animation = Анимация окон
nested = Вложенное подтверждение
context = Контекстные действия
close = Закрыть проверку
confirm = Подтвердить и вернуться
sample = Вставить многоязычный текст
close-context = Закрыть меню
editor-hint = Редактируйте текст выше. Ползунок меняет число работников.
    Tab / Shift+Tab: фокус · Enter: действие · Escape: закрыть окно
    Ctrl+A/C/X/V: выделить, копировать, вырезать, вставить · Колесо: прокрутка
