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
review = فتح المراجعة
animation = تحريك النوافذ
nested = فتح التأكيد المتداخل
context = إجراءات القائمة
close = إغلاق المراجعة
confirm = تأكيد والعودة
sample = إدراج نص متعدد اللغات
close-context = إغلاق القائمة
editor-hint = حرّر النص أعلاه. يغيّر شريط التمرير عدد العمال.
    Tab: التنقل · Enter: التفعيل · Escape: إغلاق النافذة
    Ctrl+A/C/X/V: تحديد ونسخ وقص ولصق
