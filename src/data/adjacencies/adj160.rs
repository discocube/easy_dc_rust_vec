use common_macros::hash_map;

use crate::types::types::Adj;
use crate::enums::enums::Neighbors;

pub fn adj160() -> Adj {
    let graph = hash_map! {
        0 => Neighbors::Six([1, 2, 4, 8, 12, 14]),
        1 => Neighbors::Six([0, 3, 5, 9, 13, 15]),
        2 => Neighbors::Six([0, 3, 6, 10, 16, 18]),
        3 => Neighbors::Six([1, 2, 7, 11, 17, 19]),
        4 => Neighbors::Six([0, 5, 6, 20, 22, 28]),
        5 => Neighbors::Six([1, 4, 7, 21, 23, 29]),
        6 => Neighbors::Six([2, 4, 7, 24, 26, 30]),
        7 => Neighbors::Six([3, 5, 6, 25, 27, 31]),
        8 => Neighbors::Six([0, 32, 34, 9, 10, 56]),
        9 => Neighbors::Six([1, 33, 35, 8, 11, 57]),
        10 => Neighbors::Six([2, 36, 38, 8, 11, 58]),
        11 => Neighbors::Six([3, 37, 39, 9, 10, 59]),
        12 => Neighbors::Six([32, 0, 64, 40, 13, 20]),
        13 => Neighbors::Six([65, 1, 33, 41, 12, 21]),
        14 => Neighbors::Six([0, 34, 66, 40, 16, 22]),
        15 => Neighbors::Six([1, 67, 35, 41, 17, 23]),
        16 => Neighbors::Six([2, 36, 68, 42, 14, 24]),
        17 => Neighbors::Six([3, 69, 37, 43, 15, 25]),
        18 => Neighbors::Six([2, 70, 38, 42, 19, 26]),
        19 => Neighbors::Six([3, 71, 39, 43, 18, 27]),
        20 => Neighbors::Six([4, 72, 12, 44, 48, 21]),
        21 => Neighbors::Six([5, 73, 13, 45, 49, 20]),
        22 => Neighbors::Six([4, 74, 44, 14, 50, 24]),
        23 => Neighbors::Six([5, 75, 45, 15, 51, 25]),
        24 => Neighbors::Six([6, 76, 46, 16, 52, 22]),
        25 => Neighbors::Six([7, 77, 47, 17, 53, 23]),
        26 => Neighbors::Six([6, 46, 78, 18, 54, 27]),
        27 => Neighbors::Six([7, 47, 79, 19, 55, 26]),
        28 => Neighbors::Six([4, 48, 50, 84, 29, 30]),
        29 => Neighbors::Six([5, 49, 51, 85, 28, 31]),
        30 => Neighbors::Six([6, 52, 54, 86, 28, 31]),
        31 => Neighbors::Six([7, 53, 87, 55, 29, 30]),
        32 => Neighbors::Six([96, 88, 60, 33, 8, 12]),
        33 => Neighbors::Six([97, 89, 61, 32, 9, 13]),
        34 => Neighbors::Six([98, 90, 60, 36, 8, 14]),
        35 => Neighbors::Six([99, 91, 61, 37, 9, 15]),
        36 => Neighbors::Six([100, 92, 62, 34, 10, 16]),
        37 => Neighbors::Six([101, 93, 63, 35, 11, 17]),
        38 => Neighbors::Six([102, 62, 94, 39, 10, 18]),
        39 => Neighbors::Six([103, 95, 63, 38, 11, 19]),
        40 => Neighbors::Six([104, 106, 60, 44, 12, 14]),
        41 => Neighbors::Six([105, 107, 61, 45, 13, 15]),
        42 => Neighbors::Six([108, 110, 62, 46, 16, 18]),
        43 => Neighbors::Six([109, 111, 63, 47, 17, 19]),
        44 => Neighbors::Six([80, 112, 114, 40, 20, 22]),
        45 => Neighbors::Six([81, 113, 115, 41, 21, 23]),
        46 => Neighbors::Six([82, 116, 118, 42, 24, 26]),
        47 => Neighbors::Six([83, 117, 119, 43, 25, 27]),
        48 => Neighbors::Six([128, 80, 120, 49, 20, 28]),
        49 => Neighbors::Six([129, 81, 121, 48, 21, 29]),
        50 => Neighbors::Six([130, 80, 122, 52, 22, 28]),
        51 => Neighbors::Six([131, 81, 123, 53, 23, 29]),
        52 => Neighbors::Six([132, 82, 124, 50, 24, 30]),
        53 => Neighbors::Six([133, 83, 125, 51, 25, 31]),
        54 => Neighbors::Six([134, 82, 126, 30, 55, 26]),
        55 => Neighbors::Six([135, 83, 127, 54, 27, 31]),
        56 => Neighbors::Six([90, 136, 88, 8, 57, 58]),
        57 => Neighbors::Six([137, 91, 89, 9, 56, 59]),
        58 => Neighbors::Six([138, 92, 94, 10, 56, 59]),
        59 => Neighbors::Six([139, 93, 95, 11, 57, 58]),
        60 => Neighbors::Three([40, 34, 32]),
        61 => Neighbors::Three([41, 35, 33]),
        62 => Neighbors::Three([42, 36, 38]),
        63 => Neighbors::Three([43, 37, 39]),
        64 => Neighbors::Six([96, 104, 140, 65, 72, 12]),
        65 => Neighbors::Six([97, 105, 141, 64, 73, 13]),
        66 => Neighbors::Six([98, 106, 142, 68, 74, 14]),
        67 => Neighbors::Six([99, 107, 143, 69, 75, 15]),
        68 => Neighbors::Six([100, 108, 144, 66, 76, 16]),
        69 => Neighbors::Six([101, 109, 145, 67, 77, 17]),
        70 => Neighbors::Six([102, 110, 146, 71, 78, 18]),
        71 => Neighbors::Six([103, 111, 147, 70, 79, 19]),
        72 => Neighbors::Six([112, 148, 120, 64, 73, 20]),
        73 => Neighbors::Six([113, 149, 121, 65, 72, 21]),
        74 => Neighbors::Six([114, 150, 122, 66, 76, 22]),
        75 => Neighbors::Six([115, 151, 123, 67, 77, 23]),
        76 => Neighbors::Six([116, 124, 152, 68, 74, 24]),
        77 => Neighbors::Six([117, 153, 125, 69, 75, 25]),
        78 => Neighbors::Six([154, 118, 126, 70, 79, 26]),
        79 => Neighbors::Six([155, 119, 127, 71, 78, 27]),
        80 => Neighbors::Three([48, 50, 44]),
        81 => Neighbors::Three([49, 51, 45]),
        82 => Neighbors::Three([54, 52, 46]),
        83 => Neighbors::Three([55, 53, 47]),
        84 => Neighbors::Six([128, 130, 156, 28, 85, 86]),
        85 => Neighbors::Six([129, 131, 157, 84, 87, 29]),
        86 => Neighbors::Six([132, 134, 158, 84, 30, 87]),
        87 => Neighbors::Six([159, 133, 135, 85, 86, 31]),
        88 => Neighbors::Three([89, 32, 56]),
        89 => Neighbors::Three([88, 33, 57]),
        90 => Neighbors::Three([92, 56, 34]),
        91 => Neighbors::Three([93, 57, 35]),
        92 => Neighbors::Three([90, 36, 58]),
        93 => Neighbors::Three([91, 59, 37]),
        94 => Neighbors::Three([95, 58, 38]),
        95 => Neighbors::Three([94, 59, 39]),
        96 => Neighbors::Three([97, 64, 32]),
        97 => Neighbors::Three([96, 65, 33]),
        98 => Neighbors::Three([100, 66, 34]),
        99 => Neighbors::Three([101, 35, 67]),
        100 => Neighbors::Three([98, 68, 36]),
        101 => Neighbors::Three([99, 37, 69]),
        102 => Neighbors::Three([103, 38, 70]),
        103 => Neighbors::Three([102, 39, 71]),
        104 => Neighbors::Three([112, 40, 64]),
        105 => Neighbors::Three([113, 65, 41]),
        106 => Neighbors::Three([114, 40, 66]),
        107 => Neighbors::Three([115, 41, 67]),
        108 => Neighbors::Three([116, 42, 68]),
        109 => Neighbors::Three([117, 43, 69]),
        110 => Neighbors::Three([118, 70, 42]),
        111 => Neighbors::Three([119, 43, 71]),
        112 => Neighbors::Three([104, 72, 44]),
        113 => Neighbors::Three([105, 73, 45]),
        114 => Neighbors::Three([106, 44, 74]),
        115 => Neighbors::Three([107, 75, 45]),
        116 => Neighbors::Three([108, 46, 76]),
        117 => Neighbors::Three([109, 77, 47]),
        118 => Neighbors::Three([110, 46, 78]),
        119 => Neighbors::Three([111, 47, 79]),
        120 => Neighbors::Three([121, 72, 48]),
        121 => Neighbors::Three([120, 73, 49]),
        122 => Neighbors::Three([124, 74, 50]),
        123 => Neighbors::Three([125, 51, 75]),
        124 => Neighbors::Three([122, 76, 52]),
        125 => Neighbors::Three([123, 53, 77]),
        126 => Neighbors::Three([127, 54, 78]),
        127 => Neighbors::Three([126, 55, 79]),
        128 => Neighbors::Three([129, 48, 84]),
        129 => Neighbors::Three([128, 49, 85]),
        130 => Neighbors::Three([132, 50, 84]),
        131 => Neighbors::Three([133, 51, 85]),
        132 => Neighbors::Three([130, 52, 86]),
        133 => Neighbors::Three([131, 53, 87]),
        134 => Neighbors::Three([135, 86, 54]),
        135 => Neighbors::Three([134, 87, 55]),
        136 => Neighbors::Three([137, 138, 56]),
        137 => Neighbors::Three([136, 139, 57]),
        138 => Neighbors::Three([136, 139, 58]),
        139 => Neighbors::Three([137, 138, 59]),
        140 => Neighbors::Three([148, 141, 64]),
        141 => Neighbors::Three([140, 149, 65]),
        142 => Neighbors::Three([144, 150, 66]),
        143 => Neighbors::Three([145, 151, 67]),
        144 => Neighbors::Three([152, 142, 68]),
        145 => Neighbors::Three([153, 143, 69]),
        146 => Neighbors::Three([154, 147, 70]),
        147 => Neighbors::Three([146, 155, 71]),
        148 => Neighbors::Three([140, 149, 72]),
        149 => Neighbors::Three([148, 141, 73]),
        150 => Neighbors::Three([152, 142, 74]),
        151 => Neighbors::Three([153, 143, 75]),
        152 => Neighbors::Three([144, 150, 76]),
        153 => Neighbors::Three([145, 151, 77]),
        154 => Neighbors::Three([146, 155, 78]),
        155 => Neighbors::Three([154, 147, 79]),
        156 => Neighbors::Three([157, 158, 84]),
        157 => Neighbors::Three([156, 159, 85]),
        158 => Neighbors::Three([156, 159, 86]),
        159 => Neighbors::Three([157, 158, 87]),
    };
    graph
}