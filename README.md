# Пятое задание 



Отчет должен содержать титульный лист с идентификацией студента, номер варианта, описание задачи.

В отчете необходимо подробно описать используемую модель вычислений. Привести источники информации, в которых описана данная модель.

 Описание работы с данными программы должно быть приведено в отчете и сопровождаться соотвтетсвующей информацией при работе с программой. 


## Автор
Литвинский Семён Алексеевич БПИ201 Вариант 20

## Задание

В отделе работают три программиста. Каждый
программист пишет свою программу и отдает ее на проверку другому
программисту. Программист проверяет чужую программу, когда его
собственная уже написана. По завершении проверки, программист дает
ответ: программа написана правильно или написана неправильно.

Программист спит, если не пишет свою программу и не проверяет чужую
программу. Программист просыпается, когда получает заключение от
другого программиста. Если программа признана правильной, программист
пишет другую программу, если программа признана неправильной,
программист исправляет ее и отправляет на проверку тому же программисту,
который ее проверял. Создать многопоточное приложение, моделирующее
работу программистов.

## Интерпретация

Я расширил задачу для до n программистов, где n - вводится в параметрах коммандной строки (как и файлы ввода и вывода)

Примечание: Файл вывода на момент запуска прграммы не должен существовать, программа добавит его сама.

Программист пытается взять задачу из менеджера задач, если получается то он пишет программу и отсылает её следующему (по кругу) программисту на проверку, проверяет, есть ли ответ на эту проверку. Далее, не зависимо от того, была ли у него задача, он пытается получить программу для проверки и прверить её.

Задачи, как и программы предстваляют собой числа, изменеие кода программы эквивалентно изменению её числе, едиственная корректная 
программа это 1.

## Ввод

По мимо аргументов командной строки, которые были описаны выше, есть система комманд


`manual n1 n2 n3 ...` - дает программистам тамки с соответствующим содержанием

`random` - выполняет manual с рандомными параметрами

`exit` - завершение (нужно только для консоли)

## Вывод

В командную строку или файл печатается лог событий в интуитивно понятном формате

Сообщения об ошибках по возможности пишутся в файл, только затем в консоль


## Сборка

собирается и запускается следующей командой 

`$ cargo run`