;pub const  IND_SIZE = 2;
pub const  REG_SIZE = 4;
pub const  DIR_SIZE = REG_SIZE;

pub const  REG_CODE = 1;
pub const  DIR_CODE = 2;
pub const  IND_CODE = 3;

pub const  MAX_PLAYERS = 4;
pub const  MEM_SIZE = 4 * 1024;
pub const  IDX_MOD = MEM_SIZE / 8;
pub const  PLAYER_MAX_SIZE = MEM_SIZE / 6;

pub const  COMMENT_CHAR = '#';
pub const  LABEL_CHAR = ':';
pub const  DIRECT_CHAR = '%';
pub const  SEPARATOR_CHAR = ',';

pub const  LABEL_CHARS = "abcdefghijklmnopqrstuvwxyz_0123456789";

pub const  NAME_CMD_STRING = ".name";
pub const  DESCRIPTION_CMD_STRING = ".description";

pub const  REG_NUMBER = 16;

pub const  CYCLE_TO_DIE = 1536;
pub const  CYCLE_DELTA = 50;
pub const  NBR_LIVE = 21;
pub const  MAX_CHECKS = 10;

pub const  PROG_NAME_LENGTH = 128;
pub const  DESCRIPTION_LENGTH = 2048;
pub const  COREWAR_EXEC_SIGNATURE = 0xea83f3;
