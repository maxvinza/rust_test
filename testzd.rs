
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

const ITEMS_6: &[usize] = &[183+184-18];
const RESULT_6: usize = 376;

const ITEMS_7: &[usize] = &[183+184-18+1];
const RESULT_7: usize = 564;

fn calc(items: &[usize]) -> usize {
    const CLOSE_TS:    usize =  4;
    const CLOSE_EIT:   usize =  19;
    const USEFULL_TS:  usize =  TS_PACKET_SIZE - CLOSE_TS;
    const USEFULL_EIT: usize =  EIT_SEGMENT_LIMIT  -  CLOSE_EIT;//доступные данные внутри EIT

    let mut out_summ: usize = 0;// сумма итоговая
    let mut elem:     usize;// элемент текущий
    let mut this_ts:  usize = 0;
    let mut this_eit: usize = 0;
    
    for sizes in items{
        elem = *sizes;
        if elem >= USEFULL_EIT{
            panic!("Wrong item {}!", elem);// Проверка валидности данных - не может существовать ITEM  который не поместиться в один сигмент
        }
        while elem > 0{
            if this_eit == 0{
                this_ts = CLOSE_EIT + elem;
            } else {
                this_ts += elem;                 
            }
            elem = 0;
            while this_ts >= USEFULL_TS{
                this_ts -= USEFULL_TS;
                this_eit += USEFULL_TS;
                if this_eit > USEFULL_EIT{
                    elem = *sizes;
                    this_ts = 0;
                    this_eit = 0;
                } else {
                    out_summ += 1;
                }
            }
        }
    }
    if this_ts > 0{
        out_summ += 1;
    }
    return out_summ*TS_PACKET_SIZE;
}

fn main() {
    assert_eq!(calc(ITEMS_1), RESULT_1);
    assert_eq!(calc(ITEMS_2), RESULT_2);
    assert_eq!(calc(ITEMS_3), RESULT_3);
    assert_eq!(calc(ITEMS_6), RESULT_6);
    assert_eq!(calc(ITEMS_7), RESULT_7);
    println!("Ok");
}
