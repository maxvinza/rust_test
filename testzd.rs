
const TS_PACKET_SIZE: usize = 188;
const EIT_SEGMENT_LIMIT: usize = 4096;

const ITEMS_1: &[usize] = &[195, 207];
const RESULT_1: usize = 564;

const ITEMS_2: &[usize] = &[509, 131];
const RESULT_2: usize = 752;

const ITEMS_3: &[usize] = &[
    189, 380, 1070, 324, 429, 439, 229, 387,
    933, 741, 582, 229, 203, 643, 582, 1070,
    380, 508, 459, 1072, 713, 468, 465, 336,
    459, 1072, 382, 331, 1108, 933, 599, 924,
    434, 401, 345, 324, 370, 701, 711, 599,
    468, 243, 1108, 508, 459, 1072, 131, 924,
    478, 333, 459, 1072, 890, 382, 66, 1089,
    701, 930, 643, 372, 392, 349, 342, 370,
    465, 604, 930, 465
];
const RESULT_3: usize = 40984;

fn calc(items: &[usize]) -> usize {
    const HEADLINE: usize = 14+1;//14 байт заголовок сигмента 1 - дополнительный байт в контрольной сумме
    const CTRL_SUMM: usize  = 4;
    const FOOTLINE: usize  = 4;
    
    const CTRL_EIT: usize = HEADLINE + FOOTLINE;//служебные данные внутри EIT
    
    let mut num_ts: usize = 0;// Число TS-элементов
    let mut this_ts: usize = 0;//Текущий номер TS-элемента (внутри PSI сигмента)
    let mut useful: usize;//число байт в текущем TS сигменте, которые мы можем занять полезной инфой
    let mut free_last: usize = 0;//остаток от предыдущего TS- сигмента
    let mut sizei: usize;//размер текущего Item's
    let mut sigment_size: usize = CTRL_EIT;//Общий размер текущего сигмента
    let mut item_key: usize = 0;
    let last_ts: usize = EIT_SEGMENT_LIMIT/TS_PACKET_SIZE;//Номер последнего TS сигмента
    for sizes in items{
        item_key += 1;
        useful = 0;
        sizei = *sizes;
        sigment_size += sizei + CTRL_SUMM;
        if sizei > EIT_SEGMENT_LIMIT - HEADLINE - CTRL_SUMM{
            panic!("Wrong item {}!", sizei);// Проверка валидности данных - не может существовать ITEM  который не поместиться в один сигмент
        }
        //попытаемся уместить данный ITEM в предыдущий TS-сигмент
        if sizei >= free_last {
            sizei -= free_last;//отнимаем остаток
        } else {
            free_last -= sizei;
            sizei = 0;
        }
        // если не получилось уместить в предидущий сигмент, то начинаем разделять ITEM по  TS-gfrtnfv
        while sizei >= useful {
            this_ts += 1;
            num_ts += 1;
            useful =
    	        if this_ts == 1 {
                   TS_PACKET_SIZE - HEADLINE - CTRL_SUMM // 5 байт - первый элемент сигмента и 14 байт заголовок сигмента
                } else if this_ts == last_ts || items.len() == item_key {
                   TS_PACKET_SIZE - CTRL_SUMM - FOOTLINE // для последнего сегмента добавляется контрольная сумма вкоце
                } else {
                   TS_PACKET_SIZE - CTRL_SUMM // стандартно 4 байта TS сигменат под заголовки
                };
            if this_ts >= last_ts || sigment_size >= EIT_SEGMENT_LIMIT {
                this_ts = 0;//отсчет ведем с нулевого сигмента
                sizei = *sizes;//по условиям сигмент делить нельзя, по-этому предпологаем что он не влез
                free_last = 0;//"упаковывать" кусок нового пакета в старый сигмент нельзя
                sigment_size = CTRL_EIT;
                num_ts += 2;
            } else {
                //сигмент который мы заполняемп - не последний
                if sizei >= useful{
                    //отнимаем от ITEM объем полезного места в пакете
                    sizei -= useful;
                    free_last = 0;
                } else {
                    //остаток переносим в новый сигмент
                    num_ts += 1;
                    free_last = useful-sizei;
                }
            }
        }
    }
    if free_last < CTRL_SUMM{
        num_ts += 1;//Для последнего пакета проверяем "влезет" ли контрольная сумма. Если не влезла - добавляем TS-сигмент
    }
    return num_ts*TS_PACKET_SIZE;
}

fn main() {
    assert_eq!(calc(ITEMS_1), RESULT_1);
    assert_eq!(calc(ITEMS_2), RESULT_2);
    assert_eq!(calc(ITEMS_3), RESULT_3);
    println!("Ok");
}
