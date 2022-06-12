#![allow(dead_code)]

// SPS and PPS are sent on SDP so they are not included here
pub const NALUS: [&[u8]; 120] = [
    NAL_0, NAL_1, NAL_2, NAL_3, NAL_4, NAL_5, NAL_6, NAL_7, NAL_8, NAL_9, NAL_10, NAL_11, NAL_12,
    NAL_13, NAL_14, NAL_15, NAL_16, NAL_17, NAL_18, NAL_19, NAL_20, NAL_21, NAL_22, NAL_23, NAL_24,
    NAL_25, NAL_26, NAL_27, NAL_28, NAL_29, NAL_30, NAL_31, NAL_32, NAL_33, NAL_34, NAL_35, NAL_36,
    NAL_37, NAL_38, NAL_39, NAL_40, NAL_41, NAL_42, NAL_43, NAL_44, NAL_45, NAL_46, NAL_47, NAL_48,
    NAL_49, NAL_50, NAL_51, NAL_52, NAL_53, NAL_54, NAL_55, NAL_56, NAL_57, NAL_58, NAL_59, NAL_60,
    NAL_61, NAL_62, NAL_63, NAL_64, NAL_65, NAL_66, NAL_67, NAL_68, NAL_69, NAL_70, NAL_71, NAL_72,
    NAL_73, NAL_74, NAL_75, NAL_76, NAL_77, NAL_78, NAL_79, NAL_80, NAL_81, NAL_82, NAL_83, NAL_84,
    NAL_85, NAL_86, NAL_87, NAL_88, NAL_89, NAL_90, NAL_91, NAL_92, NAL_93, NAL_94, NAL_95, NAL_96,
    NAL_97, NAL_98, NAL_99, NAL_100, NAL_101, NAL_102, NAL_103, NAL_104, NAL_105, NAL_106, NAL_107,
    NAL_108, NAL_109, NAL_110, NAL_111, NAL_112, NAL_113, NAL_114, NAL_115, NAL_116, NAL_117,
    NAL_118, NAL_119,
];

pub const NAL_SPS: &[u8] = include_bytes!("../../../../nalus/h264/sps.h264");
pub const NAL_PPS: &[u8] = include_bytes!("../../../../nalus/h264/pps.h264");

pub const NAL_0: &[u8] = include_bytes!("../../../../nalus/h264/0.h264");
pub const NAL_1: &[u8] = include_bytes!("../../../../nalus/h264/1.h264");
pub const NAL_2: &[u8] = include_bytes!("../../../../nalus/h264/2.h264");
pub const NAL_3: &[u8] = include_bytes!("../../../../nalus/h264/3.h264");
pub const NAL_4: &[u8] = include_bytes!("../../../../nalus/h264/4.h264");
pub const NAL_5: &[u8] = include_bytes!("../../../../nalus/h264/5.h264");
pub const NAL_6: &[u8] = include_bytes!("../../../../nalus/h264/6.h264");
pub const NAL_7: &[u8] = include_bytes!("../../../../nalus/h264/7.h264");
pub const NAL_8: &[u8] = include_bytes!("../../../../nalus/h264/8.h264");
pub const NAL_9: &[u8] = include_bytes!("../../../../nalus/h264/9.h264");
pub const NAL_10: &[u8] = include_bytes!("../../../../nalus/h264/10.h264");
pub const NAL_11: &[u8] = include_bytes!("../../../../nalus/h264/11.h264");
pub const NAL_12: &[u8] = include_bytes!("../../../../nalus/h264/12.h264");
pub const NAL_13: &[u8] = include_bytes!("../../../../nalus/h264/13.h264");
pub const NAL_14: &[u8] = include_bytes!("../../../../nalus/h264/14.h264");
pub const NAL_15: &[u8] = include_bytes!("../../../../nalus/h264/15.h264");
pub const NAL_16: &[u8] = include_bytes!("../../../../nalus/h264/16.h264");
pub const NAL_17: &[u8] = include_bytes!("../../../../nalus/h264/17.h264");
pub const NAL_18: &[u8] = include_bytes!("../../../../nalus/h264/18.h264");
pub const NAL_19: &[u8] = include_bytes!("../../../../nalus/h264/19.h264");
pub const NAL_20: &[u8] = include_bytes!("../../../../nalus/h264/20.h264");
pub const NAL_21: &[u8] = include_bytes!("../../../../nalus/h264/21.h264");
pub const NAL_22: &[u8] = include_bytes!("../../../../nalus/h264/22.h264");
pub const NAL_23: &[u8] = include_bytes!("../../../../nalus/h264/23.h264");
pub const NAL_24: &[u8] = include_bytes!("../../../../nalus/h264/24.h264");
pub const NAL_25: &[u8] = include_bytes!("../../../../nalus/h264/25.h264");
pub const NAL_26: &[u8] = include_bytes!("../../../../nalus/h264/26.h264");
pub const NAL_27: &[u8] = include_bytes!("../../../../nalus/h264/27.h264");
pub const NAL_28: &[u8] = include_bytes!("../../../../nalus/h264/28.h264");
pub const NAL_29: &[u8] = include_bytes!("../../../../nalus/h264/29.h264");
pub const NAL_30: &[u8] = include_bytes!("../../../../nalus/h264/30.h264");
pub const NAL_31: &[u8] = include_bytes!("../../../../nalus/h264/31.h264");
pub const NAL_32: &[u8] = include_bytes!("../../../../nalus/h264/32.h264");
pub const NAL_33: &[u8] = include_bytes!("../../../../nalus/h264/33.h264");
pub const NAL_34: &[u8] = include_bytes!("../../../../nalus/h264/34.h264");
pub const NAL_35: &[u8] = include_bytes!("../../../../nalus/h264/35.h264");
pub const NAL_36: &[u8] = include_bytes!("../../../../nalus/h264/36.h264");
pub const NAL_37: &[u8] = include_bytes!("../../../../nalus/h264/37.h264");
pub const NAL_38: &[u8] = include_bytes!("../../../../nalus/h264/38.h264");
pub const NAL_39: &[u8] = include_bytes!("../../../../nalus/h264/39.h264");
pub const NAL_40: &[u8] = include_bytes!("../../../../nalus/h264/40.h264");
pub const NAL_41: &[u8] = include_bytes!("../../../../nalus/h264/41.h264");
pub const NAL_42: &[u8] = include_bytes!("../../../../nalus/h264/42.h264");
pub const NAL_43: &[u8] = include_bytes!("../../../../nalus/h264/43.h264");
pub const NAL_44: &[u8] = include_bytes!("../../../../nalus/h264/44.h264");
pub const NAL_45: &[u8] = include_bytes!("../../../../nalus/h264/45.h264");
pub const NAL_46: &[u8] = include_bytes!("../../../../nalus/h264/46.h264");
pub const NAL_47: &[u8] = include_bytes!("../../../../nalus/h264/47.h264");
pub const NAL_48: &[u8] = include_bytes!("../../../../nalus/h264/48.h264");
pub const NAL_49: &[u8] = include_bytes!("../../../../nalus/h264/49.h264");
pub const NAL_50: &[u8] = include_bytes!("../../../../nalus/h264/50.h264");
pub const NAL_51: &[u8] = include_bytes!("../../../../nalus/h264/51.h264");
pub const NAL_52: &[u8] = include_bytes!("../../../../nalus/h264/52.h264");
pub const NAL_53: &[u8] = include_bytes!("../../../../nalus/h264/53.h264");
pub const NAL_54: &[u8] = include_bytes!("../../../../nalus/h264/54.h264");
pub const NAL_55: &[u8] = include_bytes!("../../../../nalus/h264/55.h264");
pub const NAL_56: &[u8] = include_bytes!("../../../../nalus/h264/56.h264");
pub const NAL_57: &[u8] = include_bytes!("../../../../nalus/h264/57.h264");
pub const NAL_58: &[u8] = include_bytes!("../../../../nalus/h264/58.h264");
pub const NAL_59: &[u8] = include_bytes!("../../../../nalus/h264/59.h264");
pub const NAL_60: &[u8] = include_bytes!("../../../../nalus/h264/60.h264");
pub const NAL_61: &[u8] = include_bytes!("../../../../nalus/h264/61.h264");
pub const NAL_62: &[u8] = include_bytes!("../../../../nalus/h264/62.h264");
pub const NAL_63: &[u8] = include_bytes!("../../../../nalus/h264/63.h264");
pub const NAL_64: &[u8] = include_bytes!("../../../../nalus/h264/64.h264");
pub const NAL_65: &[u8] = include_bytes!("../../../../nalus/h264/65.h264");
pub const NAL_66: &[u8] = include_bytes!("../../../../nalus/h264/66.h264");
pub const NAL_67: &[u8] = include_bytes!("../../../../nalus/h264/67.h264");
pub const NAL_68: &[u8] = include_bytes!("../../../../nalus/h264/68.h264");
pub const NAL_69: &[u8] = include_bytes!("../../../../nalus/h264/69.h264");
pub const NAL_70: &[u8] = include_bytes!("../../../../nalus/h264/70.h264");
pub const NAL_71: &[u8] = include_bytes!("../../../../nalus/h264/71.h264");
pub const NAL_72: &[u8] = include_bytes!("../../../../nalus/h264/72.h264");
pub const NAL_73: &[u8] = include_bytes!("../../../../nalus/h264/73.h264");
pub const NAL_74: &[u8] = include_bytes!("../../../../nalus/h264/74.h264");
pub const NAL_75: &[u8] = include_bytes!("../../../../nalus/h264/75.h264");
pub const NAL_76: &[u8] = include_bytes!("../../../../nalus/h264/76.h264");
pub const NAL_77: &[u8] = include_bytes!("../../../../nalus/h264/77.h264");
pub const NAL_78: &[u8] = include_bytes!("../../../../nalus/h264/78.h264");
pub const NAL_79: &[u8] = include_bytes!("../../../../nalus/h264/79.h264");
pub const NAL_80: &[u8] = include_bytes!("../../../../nalus/h264/80.h264");
pub const NAL_81: &[u8] = include_bytes!("../../../../nalus/h264/81.h264");
pub const NAL_82: &[u8] = include_bytes!("../../../../nalus/h264/82.h264");
pub const NAL_83: &[u8] = include_bytes!("../../../../nalus/h264/83.h264");
pub const NAL_84: &[u8] = include_bytes!("../../../../nalus/h264/84.h264");
pub const NAL_85: &[u8] = include_bytes!("../../../../nalus/h264/85.h264");
pub const NAL_86: &[u8] = include_bytes!("../../../../nalus/h264/86.h264");
pub const NAL_87: &[u8] = include_bytes!("../../../../nalus/h264/87.h264");
pub const NAL_88: &[u8] = include_bytes!("../../../../nalus/h264/88.h264");
pub const NAL_89: &[u8] = include_bytes!("../../../../nalus/h264/89.h264");
pub const NAL_90: &[u8] = include_bytes!("../../../../nalus/h264/90.h264");
pub const NAL_91: &[u8] = include_bytes!("../../../../nalus/h264/91.h264");
pub const NAL_92: &[u8] = include_bytes!("../../../../nalus/h264/92.h264");
pub const NAL_93: &[u8] = include_bytes!("../../../../nalus/h264/93.h264");
pub const NAL_94: &[u8] = include_bytes!("../../../../nalus/h264/94.h264");
pub const NAL_95: &[u8] = include_bytes!("../../../../nalus/h264/95.h264");
pub const NAL_96: &[u8] = include_bytes!("../../../../nalus/h264/96.h264");
pub const NAL_97: &[u8] = include_bytes!("../../../../nalus/h264/97.h264");
pub const NAL_98: &[u8] = include_bytes!("../../../../nalus/h264/98.h264");
pub const NAL_99: &[u8] = include_bytes!("../../../../nalus/h264/99.h264");
pub const NAL_100: &[u8] = include_bytes!("../../../../nalus/h264/100.h264");
pub const NAL_101: &[u8] = include_bytes!("../../../../nalus/h264/101.h264");
pub const NAL_102: &[u8] = include_bytes!("../../../../nalus/h264/102.h264");
pub const NAL_103: &[u8] = include_bytes!("../../../../nalus/h264/103.h264");
pub const NAL_104: &[u8] = include_bytes!("../../../../nalus/h264/104.h264");
pub const NAL_105: &[u8] = include_bytes!("../../../../nalus/h264/105.h264");
pub const NAL_106: &[u8] = include_bytes!("../../../../nalus/h264/106.h264");
pub const NAL_107: &[u8] = include_bytes!("../../../../nalus/h264/107.h264");
pub const NAL_108: &[u8] = include_bytes!("../../../../nalus/h264/108.h264");
pub const NAL_109: &[u8] = include_bytes!("../../../../nalus/h264/109.h264");
pub const NAL_110: &[u8] = include_bytes!("../../../../nalus/h264/110.h264");
pub const NAL_111: &[u8] = include_bytes!("../../../../nalus/h264/111.h264");
pub const NAL_112: &[u8] = include_bytes!("../../../../nalus/h264/112.h264");
pub const NAL_113: &[u8] = include_bytes!("../../../../nalus/h264/113.h264");
pub const NAL_114: &[u8] = include_bytes!("../../../../nalus/h264/114.h264");
pub const NAL_115: &[u8] = include_bytes!("../../../../nalus/h264/115.h264");
pub const NAL_116: &[u8] = include_bytes!("../../../../nalus/h264/116.h264");
pub const NAL_117: &[u8] = include_bytes!("../../../../nalus/h264/117.h264");
pub const NAL_118: &[u8] = include_bytes!("../../../../nalus/h264/118.h264");
pub const NAL_119: &[u8] = include_bytes!("../../../../nalus/h264/119.h264");
