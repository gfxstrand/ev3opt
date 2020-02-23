/* Copyright © 2020, Jason Ekstrand
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice (including the next
 * paragraph) shall be included in all copies or substantial portions of the
 * Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
 * IN THE SOFTWARE.
 */

/* This file is auto-generated via gen/parse_enums.py. DO NOT EDIT */

#![allow(dead_code)]
#![allow(non_camel_case_types)]

use std::result::Result;
use crate::ir;


#[derive(Copy, Clone)]
pub enum ProgramInfoSubcode {
    OBJ_STOP = 0,
    OBJ_START = 4,
    GET_STATUS = 22,
    GET_SPEED = 23,
    GET_PRGRESULT = 24,
}

impl ProgramInfoSubcode {
    pub fn from_u8(i: u8) -> Result<ProgramInfoSubcode, &'static str> {
        match i {
            0 => Ok(ProgramInfoSubcode::OBJ_STOP),
            4 => Ok(ProgramInfoSubcode::OBJ_START),
            22 => Ok(ProgramInfoSubcode::GET_STATUS),
            23 => Ok(ProgramInfoSubcode::GET_SPEED),
            24 => Ok(ProgramInfoSubcode::GET_PRGRESULT),
            _ => Err("Invalid enum value for ProgramInfoSubcode")
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            ProgramInfoSubcode::OBJ_STOP => "OBJ_STOP",
            ProgramInfoSubcode::OBJ_START => "OBJ_START",
            ProgramInfoSubcode::GET_STATUS => "GET_STATUS",
            ProgramInfoSubcode::GET_SPEED => "GET_SPEED",
            ProgramInfoSubcode::GET_PRGRESULT => "GET_PRGRESULT",
        }
    }

    pub fn to_u8(&self) -> u8 {
        *self as u8
    }
}

#[derive(Copy, Clone)]
pub enum InfoSubcode {
    SET_ERROR = 1,
    GET_ERROR = 2,
    ERRORTEXT = 3,
    GET_VOLUME = 4,
    SET_VOLUME = 5,
    GET_MINUTES = 6,
    SET_MINUTES = 7,
}

impl InfoSubcode {
    pub fn from_u8(i: u8) -> Result<InfoSubcode, &'static str> {
        match i {
            1 => Ok(InfoSubcode::SET_ERROR),
            2 => Ok(InfoSubcode::GET_ERROR),
            3 => Ok(InfoSubcode::ERRORTEXT),
            4 => Ok(InfoSubcode::GET_VOLUME),
            5 => Ok(InfoSubcode::SET_VOLUME),
            6 => Ok(InfoSubcode::GET_MINUTES),
            7 => Ok(InfoSubcode::SET_MINUTES),
            _ => Err("Invalid enum value for InfoSubcode")
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            InfoSubcode::SET_ERROR => "SET_ERROR",
            InfoSubcode::GET_ERROR => "GET_ERROR",
            InfoSubcode::ERRORTEXT => "ERRORTEXT",
            InfoSubcode::GET_VOLUME => "GET_VOLUME",
            InfoSubcode::SET_VOLUME => "SET_VOLUME",
            InfoSubcode::GET_MINUTES => "GET_MINUTES",
            InfoSubcode::SET_MINUTES => "SET_MINUTES",
        }
    }

    pub fn to_u8(&self) -> u8 {
        *self as u8
    }
}

#[derive(Copy, Clone)]
pub enum StringSubcode {
    GET_SIZE = 1,
    ADD = 2,
    COMPARE = 3,
    DUPLICATE = 5,
    VALUE_TO_STRING = 6,
    STRING_TO_VALUE = 7,
    STRIP = 8,
    NUMBER_TO_STRING = 9,
    SUB = 10,
    VALUE_FORMATTED = 11,
    NUMBER_FORMATTED = 12,
}

impl StringSubcode {
    pub fn from_u8(i: u8) -> Result<StringSubcode, &'static str> {
        match i {
            1 => Ok(StringSubcode::GET_SIZE),
            2 => Ok(StringSubcode::ADD),
            3 => Ok(StringSubcode::COMPARE),
            5 => Ok(StringSubcode::DUPLICATE),
            6 => Ok(StringSubcode::VALUE_TO_STRING),
            7 => Ok(StringSubcode::STRING_TO_VALUE),
            8 => Ok(StringSubcode::STRIP),
            9 => Ok(StringSubcode::NUMBER_TO_STRING),
            10 => Ok(StringSubcode::SUB),
            11 => Ok(StringSubcode::VALUE_FORMATTED),
            12 => Ok(StringSubcode::NUMBER_FORMATTED),
            _ => Err("Invalid enum value for StringSubcode")
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            StringSubcode::GET_SIZE => "GET_SIZE",
            StringSubcode::ADD => "ADD",
            StringSubcode::COMPARE => "COMPARE",
            StringSubcode::DUPLICATE => "DUPLICATE",
            StringSubcode::VALUE_TO_STRING => "VALUE_TO_STRING",
            StringSubcode::STRING_TO_VALUE => "STRING_TO_VALUE",
            StringSubcode::STRIP => "STRIP",
            StringSubcode::NUMBER_TO_STRING => "NUMBER_TO_STRING",
            StringSubcode::SUB => "SUB",
            StringSubcode::VALUE_FORMATTED => "VALUE_FORMATTED",
            StringSubcode::NUMBER_FORMATTED => "NUMBER_FORMATTED",
        }
    }

    pub fn to_u8(&self) -> u8 {
        *self as u8
    }
}

#[derive(Copy, Clone)]
pub enum UiReadSubcode {
    GET_VBATT = 1,
    GET_IBATT = 2,
    GET_OS_VERS = 3,
    GET_EVENT = 4,
    GET_TBATT = 5,
    GET_IMOTOR = 7,
    GET_STRING = 8,
    GET_HW_VERS = 9,
    GET_FW_VERS = 10,
    GET_FW_BUILD = 11,
    GET_OS_BUILD = 12,
    GET_ADDRESS = 13,
    GET_CODE = 14,
    KEY = 15,
    GET_SHUTDOWN = 16,
    GET_WARNING = 17,
    GET_LBATT = 18,
    TEXTBOX_READ = 21,
    GET_VERSION = 26,
    GET_IP = 27,
    GET_SDCARD = 30,
    GET_USBSTICK = 31,
}

impl UiReadSubcode {
    pub fn from_u8(i: u8) -> Result<UiReadSubcode, &'static str> {
        match i {
            1 => Ok(UiReadSubcode::GET_VBATT),
            2 => Ok(UiReadSubcode::GET_IBATT),
            3 => Ok(UiReadSubcode::GET_OS_VERS),
            4 => Ok(UiReadSubcode::GET_EVENT),
            5 => Ok(UiReadSubcode::GET_TBATT),
            7 => Ok(UiReadSubcode::GET_IMOTOR),
            8 => Ok(UiReadSubcode::GET_STRING),
            9 => Ok(UiReadSubcode::GET_HW_VERS),
            10 => Ok(UiReadSubcode::GET_FW_VERS),
            11 => Ok(UiReadSubcode::GET_FW_BUILD),
            12 => Ok(UiReadSubcode::GET_OS_BUILD),
            13 => Ok(UiReadSubcode::GET_ADDRESS),
            14 => Ok(UiReadSubcode::GET_CODE),
            15 => Ok(UiReadSubcode::KEY),
            16 => Ok(UiReadSubcode::GET_SHUTDOWN),
            17 => Ok(UiReadSubcode::GET_WARNING),
            18 => Ok(UiReadSubcode::GET_LBATT),
            21 => Ok(UiReadSubcode::TEXTBOX_READ),
            26 => Ok(UiReadSubcode::GET_VERSION),
            27 => Ok(UiReadSubcode::GET_IP),
            30 => Ok(UiReadSubcode::GET_SDCARD),
            31 => Ok(UiReadSubcode::GET_USBSTICK),
            _ => Err("Invalid enum value for UiReadSubcode")
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            UiReadSubcode::GET_VBATT => "GET_VBATT",
            UiReadSubcode::GET_IBATT => "GET_IBATT",
            UiReadSubcode::GET_OS_VERS => "GET_OS_VERS",
            UiReadSubcode::GET_EVENT => "GET_EVENT",
            UiReadSubcode::GET_TBATT => "GET_TBATT",
            UiReadSubcode::GET_IMOTOR => "GET_IMOTOR",
            UiReadSubcode::GET_STRING => "GET_STRING",
            UiReadSubcode::GET_HW_VERS => "GET_HW_VERS",
            UiReadSubcode::GET_FW_VERS => "GET_FW_VERS",
            UiReadSubcode::GET_FW_BUILD => "GET_FW_BUILD",
            UiReadSubcode::GET_OS_BUILD => "GET_OS_BUILD",
            UiReadSubcode::GET_ADDRESS => "GET_ADDRESS",
            UiReadSubcode::GET_CODE => "GET_CODE",
            UiReadSubcode::KEY => "KEY",
            UiReadSubcode::GET_SHUTDOWN => "GET_SHUTDOWN",
            UiReadSubcode::GET_WARNING => "GET_WARNING",
            UiReadSubcode::GET_LBATT => "GET_LBATT",
            UiReadSubcode::TEXTBOX_READ => "TEXTBOX_READ",
            UiReadSubcode::GET_VERSION => "GET_VERSION",
            UiReadSubcode::GET_IP => "GET_IP",
            UiReadSubcode::GET_SDCARD => "GET_SDCARD",
            UiReadSubcode::GET_USBSTICK => "GET_USBSTICK",
        }
    }

    pub fn to_u8(&self) -> u8 {
        *self as u8
    }
}

#[derive(Copy, Clone)]
pub enum UiWriteSubcode {
    WRITE_FLUSH = 1,
    FLOATVALUE = 2,
    PUT_STRING = 8,
    VALUE8 = 9,
    VALUE16 = 10,
    VALUE32 = 11,
    VALUEF = 12,
    DOWNLOAD_END = 15,
    SCREEN_BLOCK = 16,
    TEXTBOX_APPEND = 21,
    SET_BUSY = 22,
    SET_TESTPIN = 24,
    INIT_RUN = 25,
    LED = 27,
    POWER = 29,
    GRAPH_SAMPLE = 30,
    TERMINAL = 31,
}

impl UiWriteSubcode {
    pub fn from_u8(i: u8) -> Result<UiWriteSubcode, &'static str> {
        match i {
            1 => Ok(UiWriteSubcode::WRITE_FLUSH),
            2 => Ok(UiWriteSubcode::FLOATVALUE),
            8 => Ok(UiWriteSubcode::PUT_STRING),
            9 => Ok(UiWriteSubcode::VALUE8),
            10 => Ok(UiWriteSubcode::VALUE16),
            11 => Ok(UiWriteSubcode::VALUE32),
            12 => Ok(UiWriteSubcode::VALUEF),
            15 => Ok(UiWriteSubcode::DOWNLOAD_END),
            16 => Ok(UiWriteSubcode::SCREEN_BLOCK),
            21 => Ok(UiWriteSubcode::TEXTBOX_APPEND),
            22 => Ok(UiWriteSubcode::SET_BUSY),
            24 => Ok(UiWriteSubcode::SET_TESTPIN),
            25 => Ok(UiWriteSubcode::INIT_RUN),
            27 => Ok(UiWriteSubcode::LED),
            29 => Ok(UiWriteSubcode::POWER),
            30 => Ok(UiWriteSubcode::GRAPH_SAMPLE),
            31 => Ok(UiWriteSubcode::TERMINAL),
            _ => Err("Invalid enum value for UiWriteSubcode")
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            UiWriteSubcode::WRITE_FLUSH => "WRITE_FLUSH",
            UiWriteSubcode::FLOATVALUE => "FLOATVALUE",
            UiWriteSubcode::PUT_STRING => "PUT_STRING",
            UiWriteSubcode::VALUE8 => "VALUE8",
            UiWriteSubcode::VALUE16 => "VALUE16",
            UiWriteSubcode::VALUE32 => "VALUE32",
            UiWriteSubcode::VALUEF => "VALUEF",
            UiWriteSubcode::DOWNLOAD_END => "DOWNLOAD_END",
            UiWriteSubcode::SCREEN_BLOCK => "SCREEN_BLOCK",
            UiWriteSubcode::TEXTBOX_APPEND => "TEXTBOX_APPEND",
            UiWriteSubcode::SET_BUSY => "SET_BUSY",
            UiWriteSubcode::SET_TESTPIN => "SET_TESTPIN",
            UiWriteSubcode::INIT_RUN => "INIT_RUN",
            UiWriteSubcode::LED => "LED",
            UiWriteSubcode::POWER => "POWER",
            UiWriteSubcode::GRAPH_SAMPLE => "GRAPH_SAMPLE",
            UiWriteSubcode::TERMINAL => "TERMINAL",
        }
    }

    pub fn to_u8(&self) -> u8 {
        *self as u8
    }
}

#[derive(Copy, Clone)]
pub enum UiButtonSubcode {
    SHORTPRESS = 1,
    LONGPRESS = 2,
    WAIT_FOR_PRESS = 3,
    FLUSH = 4,
    PRESS = 5,
    RELEASE = 6,
    GET_HORZ = 7,
    GET_VERT = 8,
    PRESSED = 9,
    SET_BACK_BLOCK = 10,
    GET_BACK_BLOCK = 11,
    TESTSHORTPRESS = 12,
    TESTLONGPRESS = 13,
    GET_BUMBED = 14,
    GET_CLICK = 15,
}

impl UiButtonSubcode {
    pub fn from_u8(i: u8) -> Result<UiButtonSubcode, &'static str> {
        match i {
            1 => Ok(UiButtonSubcode::SHORTPRESS),
            2 => Ok(UiButtonSubcode::LONGPRESS),
            3 => Ok(UiButtonSubcode::WAIT_FOR_PRESS),
            4 => Ok(UiButtonSubcode::FLUSH),
            5 => Ok(UiButtonSubcode::PRESS),
            6 => Ok(UiButtonSubcode::RELEASE),
            7 => Ok(UiButtonSubcode::GET_HORZ),
            8 => Ok(UiButtonSubcode::GET_VERT),
            9 => Ok(UiButtonSubcode::PRESSED),
            10 => Ok(UiButtonSubcode::SET_BACK_BLOCK),
            11 => Ok(UiButtonSubcode::GET_BACK_BLOCK),
            12 => Ok(UiButtonSubcode::TESTSHORTPRESS),
            13 => Ok(UiButtonSubcode::TESTLONGPRESS),
            14 => Ok(UiButtonSubcode::GET_BUMBED),
            15 => Ok(UiButtonSubcode::GET_CLICK),
            _ => Err("Invalid enum value for UiButtonSubcode")
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            UiButtonSubcode::SHORTPRESS => "SHORTPRESS",
            UiButtonSubcode::LONGPRESS => "LONGPRESS",
            UiButtonSubcode::WAIT_FOR_PRESS => "WAIT_FOR_PRESS",
            UiButtonSubcode::FLUSH => "FLUSH",
            UiButtonSubcode::PRESS => "PRESS",
            UiButtonSubcode::RELEASE => "RELEASE",
            UiButtonSubcode::GET_HORZ => "GET_HORZ",
            UiButtonSubcode::GET_VERT => "GET_VERT",
            UiButtonSubcode::PRESSED => "PRESSED",
            UiButtonSubcode::SET_BACK_BLOCK => "SET_BACK_BLOCK",
            UiButtonSubcode::GET_BACK_BLOCK => "GET_BACK_BLOCK",
            UiButtonSubcode::TESTSHORTPRESS => "TESTSHORTPRESS",
            UiButtonSubcode::TESTLONGPRESS => "TESTLONGPRESS",
            UiButtonSubcode::GET_BUMBED => "GET_BUMBED",
            UiButtonSubcode::GET_CLICK => "GET_CLICK",
        }
    }

    pub fn to_u8(&self) -> u8 {
        *self as u8
    }
}

#[derive(Copy, Clone)]
pub enum UiDrawSubcode {
    UPDATE = 0,
    PIXEL = 2,
    LINE = 3,
    CIRCLE = 4,
    TEXT = 5,
    ICON = 6,
    PICTURE = 7,
    VALUE = 8,
    FILLRECT = 9,
    RECT = 10,
    NOTIFICATION = 11,
    QUESTION = 12,
    KEYBOARD = 13,
    BROWSE = 14,
    VERTBAR = 15,
    INVERSERECT = 16,
    SELECT_FONT = 17,
    TOPLINE = 18,
    FILLWINDOW = 19,
    DOTLINE = 21,
    VIEW_VALUE = 22,
    VIEW_UNIT = 23,
    FILLCIRCLE = 24,
    STORE = 25,
    RESTORE = 26,
    ICON_QUESTION = 27,
    BMPFILE = 28,
    GRAPH_SETUP = 30,
    GRAPH_DRAW = 31,
    TEXTBOX = 32,
}

impl UiDrawSubcode {
    pub fn from_u8(i: u8) -> Result<UiDrawSubcode, &'static str> {
        match i {
            0 => Ok(UiDrawSubcode::UPDATE),
            2 => Ok(UiDrawSubcode::PIXEL),
            3 => Ok(UiDrawSubcode::LINE),
            4 => Ok(UiDrawSubcode::CIRCLE),
            5 => Ok(UiDrawSubcode::TEXT),
            6 => Ok(UiDrawSubcode::ICON),
            7 => Ok(UiDrawSubcode::PICTURE),
            8 => Ok(UiDrawSubcode::VALUE),
            9 => Ok(UiDrawSubcode::FILLRECT),
            10 => Ok(UiDrawSubcode::RECT),
            11 => Ok(UiDrawSubcode::NOTIFICATION),
            12 => Ok(UiDrawSubcode::QUESTION),
            13 => Ok(UiDrawSubcode::KEYBOARD),
            14 => Ok(UiDrawSubcode::BROWSE),
            15 => Ok(UiDrawSubcode::VERTBAR),
            16 => Ok(UiDrawSubcode::INVERSERECT),
            17 => Ok(UiDrawSubcode::SELECT_FONT),
            18 => Ok(UiDrawSubcode::TOPLINE),
            19 => Ok(UiDrawSubcode::FILLWINDOW),
            21 => Ok(UiDrawSubcode::DOTLINE),
            22 => Ok(UiDrawSubcode::VIEW_VALUE),
            23 => Ok(UiDrawSubcode::VIEW_UNIT),
            24 => Ok(UiDrawSubcode::FILLCIRCLE),
            25 => Ok(UiDrawSubcode::STORE),
            26 => Ok(UiDrawSubcode::RESTORE),
            27 => Ok(UiDrawSubcode::ICON_QUESTION),
            28 => Ok(UiDrawSubcode::BMPFILE),
            30 => Ok(UiDrawSubcode::GRAPH_SETUP),
            31 => Ok(UiDrawSubcode::GRAPH_DRAW),
            32 => Ok(UiDrawSubcode::TEXTBOX),
            _ => Err("Invalid enum value for UiDrawSubcode")
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            UiDrawSubcode::UPDATE => "UPDATE",
            UiDrawSubcode::PIXEL => "PIXEL",
            UiDrawSubcode::LINE => "LINE",
            UiDrawSubcode::CIRCLE => "CIRCLE",
            UiDrawSubcode::TEXT => "TEXT",
            UiDrawSubcode::ICON => "ICON",
            UiDrawSubcode::PICTURE => "PICTURE",
            UiDrawSubcode::VALUE => "VALUE",
            UiDrawSubcode::FILLRECT => "FILLRECT",
            UiDrawSubcode::RECT => "RECT",
            UiDrawSubcode::NOTIFICATION => "NOTIFICATION",
            UiDrawSubcode::QUESTION => "QUESTION",
            UiDrawSubcode::KEYBOARD => "KEYBOARD",
            UiDrawSubcode::BROWSE => "BROWSE",
            UiDrawSubcode::VERTBAR => "VERTBAR",
            UiDrawSubcode::INVERSERECT => "INVERSERECT",
            UiDrawSubcode::SELECT_FONT => "SELECT_FONT",
            UiDrawSubcode::TOPLINE => "TOPLINE",
            UiDrawSubcode::FILLWINDOW => "FILLWINDOW",
            UiDrawSubcode::DOTLINE => "DOTLINE",
            UiDrawSubcode::VIEW_VALUE => "VIEW_VALUE",
            UiDrawSubcode::VIEW_UNIT => "VIEW_UNIT",
            UiDrawSubcode::FILLCIRCLE => "FILLCIRCLE",
            UiDrawSubcode::STORE => "STORE",
            UiDrawSubcode::RESTORE => "RESTORE",
            UiDrawSubcode::ICON_QUESTION => "ICON_QUESTION",
            UiDrawSubcode::BMPFILE => "BMPFILE",
            UiDrawSubcode::GRAPH_SETUP => "GRAPH_SETUP",
            UiDrawSubcode::GRAPH_DRAW => "GRAPH_DRAW",
            UiDrawSubcode::TEXTBOX => "TEXTBOX",
        }
    }

    pub fn to_u8(&self) -> u8 {
        *self as u8
    }
}

#[derive(Copy, Clone)]
pub enum MathSubcode {
    EXP = 1,
    MOD = 2,
    FLOOR = 3,
    CEIL = 4,
    ROUND = 5,
    ABS = 6,
    NEGATE = 7,
    SQRT = 8,
    LOG = 9,
    LN = 10,
    SIN = 11,
    COS = 12,
    TAN = 13,
    ASIN = 14,
    ACOS = 15,
    ATAN = 16,
    MOD8 = 17,
    MOD16 = 18,
    MOD32 = 19,
    POW = 20,
    TRUNC = 21,
}

impl MathSubcode {
    pub fn from_u8(i: u8) -> Result<MathSubcode, &'static str> {
        match i {
            1 => Ok(MathSubcode::EXP),
            2 => Ok(MathSubcode::MOD),
            3 => Ok(MathSubcode::FLOOR),
            4 => Ok(MathSubcode::CEIL),
            5 => Ok(MathSubcode::ROUND),
            6 => Ok(MathSubcode::ABS),
            7 => Ok(MathSubcode::NEGATE),
            8 => Ok(MathSubcode::SQRT),
            9 => Ok(MathSubcode::LOG),
            10 => Ok(MathSubcode::LN),
            11 => Ok(MathSubcode::SIN),
            12 => Ok(MathSubcode::COS),
            13 => Ok(MathSubcode::TAN),
            14 => Ok(MathSubcode::ASIN),
            15 => Ok(MathSubcode::ACOS),
            16 => Ok(MathSubcode::ATAN),
            17 => Ok(MathSubcode::MOD8),
            18 => Ok(MathSubcode::MOD16),
            19 => Ok(MathSubcode::MOD32),
            20 => Ok(MathSubcode::POW),
            21 => Ok(MathSubcode::TRUNC),
            _ => Err("Invalid enum value for MathSubcode")
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            MathSubcode::EXP => "EXP",
            MathSubcode::MOD => "MOD",
            MathSubcode::FLOOR => "FLOOR",
            MathSubcode::CEIL => "CEIL",
            MathSubcode::ROUND => "ROUND",
            MathSubcode::ABS => "ABS",
            MathSubcode::NEGATE => "NEGATE",
            MathSubcode::SQRT => "SQRT",
            MathSubcode::LOG => "LOG",
            MathSubcode::LN => "LN",
            MathSubcode::SIN => "SIN",
            MathSubcode::COS => "COS",
            MathSubcode::TAN => "TAN",
            MathSubcode::ASIN => "ASIN",
            MathSubcode::ACOS => "ACOS",
            MathSubcode::ATAN => "ATAN",
            MathSubcode::MOD8 => "MOD8",
            MathSubcode::MOD16 => "MOD16",
            MathSubcode::MOD32 => "MOD32",
            MathSubcode::POW => "POW",
            MathSubcode::TRUNC => "TRUNC",
        }
    }

    pub fn to_u8(&self) -> u8 {
        *self as u8
    }
}

#[derive(Copy, Clone)]
pub enum ComReadSubcode {
    COMMAND = 14,
}

impl ComReadSubcode {
    pub fn from_u8(i: u8) -> Result<ComReadSubcode, &'static str> {
        match i {
            14 => Ok(ComReadSubcode::COMMAND),
            _ => Err("Invalid enum value for ComReadSubcode")
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            ComReadSubcode::COMMAND => "COMMAND",
        }
    }

    pub fn to_u8(&self) -> u8 {
        *self as u8
    }
}

#[derive(Copy, Clone)]
pub enum ComWriteSubcode {
    REPLY = 14,
}

impl ComWriteSubcode {
    pub fn from_u8(i: u8) -> Result<ComWriteSubcode, &'static str> {
        match i {
            14 => Ok(ComWriteSubcode::REPLY),
            _ => Err("Invalid enum value for ComWriteSubcode")
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            ComWriteSubcode::REPLY => "REPLY",
        }
    }

    pub fn to_u8(&self) -> u8 {
        *self as u8
    }
}

#[derive(Copy, Clone)]
pub enum SoundSubcode {
    BREAK = 0,
    TONE = 1,
    PLAY = 2,
    REPEAT = 3,
}

impl SoundSubcode {
    pub fn from_u8(i: u8) -> Result<SoundSubcode, &'static str> {
        match i {
            0 => Ok(SoundSubcode::BREAK),
            1 => Ok(SoundSubcode::TONE),
            2 => Ok(SoundSubcode::PLAY),
            3 => Ok(SoundSubcode::REPEAT),
            _ => Err("Invalid enum value for SoundSubcode")
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            SoundSubcode::BREAK => "BREAK",
            SoundSubcode::TONE => "TONE",
            SoundSubcode::PLAY => "PLAY",
            SoundSubcode::REPEAT => "REPEAT",
        }
    }

    pub fn to_u8(&self) -> u8 {
        *self as u8
    }
}

#[derive(Copy, Clone)]
pub enum InputDeviceSubcode {
    GET_FORMAT = 2,
    CAL_MINMAX = 3,
    CAL_DEFAULT = 4,
    GET_TYPEMODE = 5,
    GET_SYMBOL = 6,
    CAL_MIN = 7,
    CAL_MAX = 8,
    SETUP = 9,
    CLR_ALL = 10,
    GET_RAW = 11,
    GET_CONNECTION = 12,
    STOP_ALL = 13,
    GET_NAME = 21,
    GET_MODENAME = 22,
    GET_FIGURES = 24,
    GET_CHANGES = 25,
    CLR_CHANGES = 26,
    READY_PCT = 27,
    READY_RAW = 28,
    READY_SI = 29,
    GET_MINMAX = 30,
    GET_BUMPS = 31,
}

impl InputDeviceSubcode {
    pub fn from_u8(i: u8) -> Result<InputDeviceSubcode, &'static str> {
        match i {
            2 => Ok(InputDeviceSubcode::GET_FORMAT),
            3 => Ok(InputDeviceSubcode::CAL_MINMAX),
            4 => Ok(InputDeviceSubcode::CAL_DEFAULT),
            5 => Ok(InputDeviceSubcode::GET_TYPEMODE),
            6 => Ok(InputDeviceSubcode::GET_SYMBOL),
            7 => Ok(InputDeviceSubcode::CAL_MIN),
            8 => Ok(InputDeviceSubcode::CAL_MAX),
            9 => Ok(InputDeviceSubcode::SETUP),
            10 => Ok(InputDeviceSubcode::CLR_ALL),
            11 => Ok(InputDeviceSubcode::GET_RAW),
            12 => Ok(InputDeviceSubcode::GET_CONNECTION),
            13 => Ok(InputDeviceSubcode::STOP_ALL),
            21 => Ok(InputDeviceSubcode::GET_NAME),
            22 => Ok(InputDeviceSubcode::GET_MODENAME),
            24 => Ok(InputDeviceSubcode::GET_FIGURES),
            25 => Ok(InputDeviceSubcode::GET_CHANGES),
            26 => Ok(InputDeviceSubcode::CLR_CHANGES),
            27 => Ok(InputDeviceSubcode::READY_PCT),
            28 => Ok(InputDeviceSubcode::READY_RAW),
            29 => Ok(InputDeviceSubcode::READY_SI),
            30 => Ok(InputDeviceSubcode::GET_MINMAX),
            31 => Ok(InputDeviceSubcode::GET_BUMPS),
            _ => Err("Invalid enum value for InputDeviceSubcode")
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            InputDeviceSubcode::GET_FORMAT => "GET_FORMAT",
            InputDeviceSubcode::CAL_MINMAX => "CAL_MINMAX",
            InputDeviceSubcode::CAL_DEFAULT => "CAL_DEFAULT",
            InputDeviceSubcode::GET_TYPEMODE => "GET_TYPEMODE",
            InputDeviceSubcode::GET_SYMBOL => "GET_SYMBOL",
            InputDeviceSubcode::CAL_MIN => "CAL_MIN",
            InputDeviceSubcode::CAL_MAX => "CAL_MAX",
            InputDeviceSubcode::SETUP => "SETUP",
            InputDeviceSubcode::CLR_ALL => "CLR_ALL",
            InputDeviceSubcode::GET_RAW => "GET_RAW",
            InputDeviceSubcode::GET_CONNECTION => "GET_CONNECTION",
            InputDeviceSubcode::STOP_ALL => "STOP_ALL",
            InputDeviceSubcode::GET_NAME => "GET_NAME",
            InputDeviceSubcode::GET_MODENAME => "GET_MODENAME",
            InputDeviceSubcode::GET_FIGURES => "GET_FIGURES",
            InputDeviceSubcode::GET_CHANGES => "GET_CHANGES",
            InputDeviceSubcode::CLR_CHANGES => "CLR_CHANGES",
            InputDeviceSubcode::READY_PCT => "READY_PCT",
            InputDeviceSubcode::READY_RAW => "READY_RAW",
            InputDeviceSubcode::READY_SI => "READY_SI",
            InputDeviceSubcode::GET_MINMAX => "GET_MINMAX",
            InputDeviceSubcode::GET_BUMPS => "GET_BUMPS",
        }
    }

    pub fn to_u8(&self) -> u8 {
        *self as u8
    }
}

#[derive(Copy, Clone)]
pub enum FileSubcode {
    OPEN_APPEND = 0,
    OPEN_READ = 1,
    OPEN_WRITE = 2,
    READ_VALUE = 3,
    WRITE_VALUE = 4,
    READ_TEXT = 5,
    WRITE_TEXT = 6,
    CLOSE = 7,
    LOAD_IMAGE = 8,
    GET_HANDLE = 9,
    MAKE_FOLDER = 10,
    GET_POOL = 11,
    SET_LOG_SYNC_TIME = 12,
    GET_FOLDERS = 13,
    GET_LOG_SYNC_TIME = 14,
    GET_SUBFOLDER_NAME = 15,
    WRITE_LOG = 16,
    CLOSE_LOG = 17,
    GET_IMAGE = 18,
    GET_ITEM = 19,
    GET_CACHE_FILES = 20,
    PUT_CACHE_FILE = 21,
    GET_CACHE_FILE = 22,
    DEL_CACHE_FILE = 23,
    DEL_SUBFOLDER = 24,
    GET_LOG_NAME = 25,
    OPEN_LOG = 27,
    READ_BYTES = 28,
    WRITE_BYTES = 29,
    REMOVE = 30,
    MOVE = 31,
}

impl FileSubcode {
    pub fn from_u8(i: u8) -> Result<FileSubcode, &'static str> {
        match i {
            0 => Ok(FileSubcode::OPEN_APPEND),
            1 => Ok(FileSubcode::OPEN_READ),
            2 => Ok(FileSubcode::OPEN_WRITE),
            3 => Ok(FileSubcode::READ_VALUE),
            4 => Ok(FileSubcode::WRITE_VALUE),
            5 => Ok(FileSubcode::READ_TEXT),
            6 => Ok(FileSubcode::WRITE_TEXT),
            7 => Ok(FileSubcode::CLOSE),
            8 => Ok(FileSubcode::LOAD_IMAGE),
            9 => Ok(FileSubcode::GET_HANDLE),
            10 => Ok(FileSubcode::MAKE_FOLDER),
            11 => Ok(FileSubcode::GET_POOL),
            12 => Ok(FileSubcode::SET_LOG_SYNC_TIME),
            13 => Ok(FileSubcode::GET_FOLDERS),
            14 => Ok(FileSubcode::GET_LOG_SYNC_TIME),
            15 => Ok(FileSubcode::GET_SUBFOLDER_NAME),
            16 => Ok(FileSubcode::WRITE_LOG),
            17 => Ok(FileSubcode::CLOSE_LOG),
            18 => Ok(FileSubcode::GET_IMAGE),
            19 => Ok(FileSubcode::GET_ITEM),
            20 => Ok(FileSubcode::GET_CACHE_FILES),
            21 => Ok(FileSubcode::PUT_CACHE_FILE),
            22 => Ok(FileSubcode::GET_CACHE_FILE),
            23 => Ok(FileSubcode::DEL_CACHE_FILE),
            24 => Ok(FileSubcode::DEL_SUBFOLDER),
            25 => Ok(FileSubcode::GET_LOG_NAME),
            27 => Ok(FileSubcode::OPEN_LOG),
            28 => Ok(FileSubcode::READ_BYTES),
            29 => Ok(FileSubcode::WRITE_BYTES),
            30 => Ok(FileSubcode::REMOVE),
            31 => Ok(FileSubcode::MOVE),
            _ => Err("Invalid enum value for FileSubcode")
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            FileSubcode::OPEN_APPEND => "OPEN_APPEND",
            FileSubcode::OPEN_READ => "OPEN_READ",
            FileSubcode::OPEN_WRITE => "OPEN_WRITE",
            FileSubcode::READ_VALUE => "READ_VALUE",
            FileSubcode::WRITE_VALUE => "WRITE_VALUE",
            FileSubcode::READ_TEXT => "READ_TEXT",
            FileSubcode::WRITE_TEXT => "WRITE_TEXT",
            FileSubcode::CLOSE => "CLOSE",
            FileSubcode::LOAD_IMAGE => "LOAD_IMAGE",
            FileSubcode::GET_HANDLE => "GET_HANDLE",
            FileSubcode::MAKE_FOLDER => "MAKE_FOLDER",
            FileSubcode::GET_POOL => "GET_POOL",
            FileSubcode::SET_LOG_SYNC_TIME => "SET_LOG_SYNC_TIME",
            FileSubcode::GET_FOLDERS => "GET_FOLDERS",
            FileSubcode::GET_LOG_SYNC_TIME => "GET_LOG_SYNC_TIME",
            FileSubcode::GET_SUBFOLDER_NAME => "GET_SUBFOLDER_NAME",
            FileSubcode::WRITE_LOG => "WRITE_LOG",
            FileSubcode::CLOSE_LOG => "CLOSE_LOG",
            FileSubcode::GET_IMAGE => "GET_IMAGE",
            FileSubcode::GET_ITEM => "GET_ITEM",
            FileSubcode::GET_CACHE_FILES => "GET_CACHE_FILES",
            FileSubcode::PUT_CACHE_FILE => "PUT_CACHE_FILE",
            FileSubcode::GET_CACHE_FILE => "GET_CACHE_FILE",
            FileSubcode::DEL_CACHE_FILE => "DEL_CACHE_FILE",
            FileSubcode::DEL_SUBFOLDER => "DEL_SUBFOLDER",
            FileSubcode::GET_LOG_NAME => "GET_LOG_NAME",
            FileSubcode::OPEN_LOG => "OPEN_LOG",
            FileSubcode::READ_BYTES => "READ_BYTES",
            FileSubcode::WRITE_BYTES => "WRITE_BYTES",
            FileSubcode::REMOVE => "REMOVE",
            FileSubcode::MOVE => "MOVE",
        }
    }

    pub fn to_u8(&self) -> u8 {
        *self as u8
    }
}

#[derive(Copy, Clone)]
pub enum ArraySubcode {
    DELETE = 0,
    CREATE8 = 1,
    CREATE16 = 2,
    CREATE32 = 3,
    CREATEF = 4,
    RESIZE = 5,
    FILL = 6,
    COPY = 7,
    INIT8 = 8,
    INIT16 = 9,
    INIT32 = 10,
    INITF = 11,
    SIZE = 12,
    READ_CONTENT = 13,
    WRITE_CONTENT = 14,
    READ_SIZE = 15,
}

impl ArraySubcode {
    pub fn from_u8(i: u8) -> Result<ArraySubcode, &'static str> {
        match i {
            0 => Ok(ArraySubcode::DELETE),
            1 => Ok(ArraySubcode::CREATE8),
            2 => Ok(ArraySubcode::CREATE16),
            3 => Ok(ArraySubcode::CREATE32),
            4 => Ok(ArraySubcode::CREATEF),
            5 => Ok(ArraySubcode::RESIZE),
            6 => Ok(ArraySubcode::FILL),
            7 => Ok(ArraySubcode::COPY),
            8 => Ok(ArraySubcode::INIT8),
            9 => Ok(ArraySubcode::INIT16),
            10 => Ok(ArraySubcode::INIT32),
            11 => Ok(ArraySubcode::INITF),
            12 => Ok(ArraySubcode::SIZE),
            13 => Ok(ArraySubcode::READ_CONTENT),
            14 => Ok(ArraySubcode::WRITE_CONTENT),
            15 => Ok(ArraySubcode::READ_SIZE),
            _ => Err("Invalid enum value for ArraySubcode")
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            ArraySubcode::DELETE => "DELETE",
            ArraySubcode::CREATE8 => "CREATE8",
            ArraySubcode::CREATE16 => "CREATE16",
            ArraySubcode::CREATE32 => "CREATE32",
            ArraySubcode::CREATEF => "CREATEF",
            ArraySubcode::RESIZE => "RESIZE",
            ArraySubcode::FILL => "FILL",
            ArraySubcode::COPY => "COPY",
            ArraySubcode::INIT8 => "INIT8",
            ArraySubcode::INIT16 => "INIT16",
            ArraySubcode::INIT32 => "INIT32",
            ArraySubcode::INITF => "INITF",
            ArraySubcode::SIZE => "SIZE",
            ArraySubcode::READ_CONTENT => "READ_CONTENT",
            ArraySubcode::WRITE_CONTENT => "WRITE_CONTENT",
            ArraySubcode::READ_SIZE => "READ_SIZE",
        }
    }

    pub fn to_u8(&self) -> u8 {
        *self as u8
    }
}

#[derive(Copy, Clone)]
pub enum FilenameSubcode {
    EXIST = 16,
    TOTALSIZE = 17,
    SPLIT = 18,
    MERGE = 19,
    CHECK = 20,
    PACK = 21,
    UNPACK = 22,
    GET_FOLDERNAME = 23,
}

impl FilenameSubcode {
    pub fn from_u8(i: u8) -> Result<FilenameSubcode, &'static str> {
        match i {
            16 => Ok(FilenameSubcode::EXIST),
            17 => Ok(FilenameSubcode::TOTALSIZE),
            18 => Ok(FilenameSubcode::SPLIT),
            19 => Ok(FilenameSubcode::MERGE),
            20 => Ok(FilenameSubcode::CHECK),
            21 => Ok(FilenameSubcode::PACK),
            22 => Ok(FilenameSubcode::UNPACK),
            23 => Ok(FilenameSubcode::GET_FOLDERNAME),
            _ => Err("Invalid enum value for FilenameSubcode")
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            FilenameSubcode::EXIST => "EXIST",
            FilenameSubcode::TOTALSIZE => "TOTALSIZE",
            FilenameSubcode::SPLIT => "SPLIT",
            FilenameSubcode::MERGE => "MERGE",
            FilenameSubcode::CHECK => "CHECK",
            FilenameSubcode::PACK => "PACK",
            FilenameSubcode::UNPACK => "UNPACK",
            FilenameSubcode::GET_FOLDERNAME => "GET_FOLDERNAME",
        }
    }

    pub fn to_u8(&self) -> u8 {
        *self as u8
    }
}

#[derive(Copy, Clone)]
pub enum ComGetSubcode {
    GET_ON_OFF = 1,
    GET_VISIBLE = 2,
    GET_RESULT = 4,
    GET_PIN = 5,
    SEARCH_ITEMS = 8,
    SEARCH_ITEM = 9,
    FAVOUR_ITEMS = 10,
    FAVOUR_ITEM = 11,
    GET_ID = 12,
    GET_BRICKNAME = 13,
    GET_NETWORK = 14,
    GET_PRESENT = 15,
    GET_ENCRYPT = 16,
    GET_INCOMING = 19,
}

impl ComGetSubcode {
    pub fn from_u8(i: u8) -> Result<ComGetSubcode, &'static str> {
        match i {
            1 => Ok(ComGetSubcode::GET_ON_OFF),
            2 => Ok(ComGetSubcode::GET_VISIBLE),
            4 => Ok(ComGetSubcode::GET_RESULT),
            5 => Ok(ComGetSubcode::GET_PIN),
            8 => Ok(ComGetSubcode::SEARCH_ITEMS),
            9 => Ok(ComGetSubcode::SEARCH_ITEM),
            10 => Ok(ComGetSubcode::FAVOUR_ITEMS),
            11 => Ok(ComGetSubcode::FAVOUR_ITEM),
            12 => Ok(ComGetSubcode::GET_ID),
            13 => Ok(ComGetSubcode::GET_BRICKNAME),
            14 => Ok(ComGetSubcode::GET_NETWORK),
            15 => Ok(ComGetSubcode::GET_PRESENT),
            16 => Ok(ComGetSubcode::GET_ENCRYPT),
            19 => Ok(ComGetSubcode::GET_INCOMING),
            _ => Err("Invalid enum value for ComGetSubcode")
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            ComGetSubcode::GET_ON_OFF => "GET_ON_OFF",
            ComGetSubcode::GET_VISIBLE => "GET_VISIBLE",
            ComGetSubcode::GET_RESULT => "GET_RESULT",
            ComGetSubcode::GET_PIN => "GET_PIN",
            ComGetSubcode::SEARCH_ITEMS => "SEARCH_ITEMS",
            ComGetSubcode::SEARCH_ITEM => "SEARCH_ITEM",
            ComGetSubcode::FAVOUR_ITEMS => "FAVOUR_ITEMS",
            ComGetSubcode::FAVOUR_ITEM => "FAVOUR_ITEM",
            ComGetSubcode::GET_ID => "GET_ID",
            ComGetSubcode::GET_BRICKNAME => "GET_BRICKNAME",
            ComGetSubcode::GET_NETWORK => "GET_NETWORK",
            ComGetSubcode::GET_PRESENT => "GET_PRESENT",
            ComGetSubcode::GET_ENCRYPT => "GET_ENCRYPT",
            ComGetSubcode::GET_INCOMING => "GET_INCOMING",
        }
    }

    pub fn to_u8(&self) -> u8 {
        *self as u8
    }
}

#[derive(Copy, Clone)]
pub enum ComSetSubcode {
    SET_ON_OFF = 1,
    SET_VISIBLE = 2,
    SET_SEARCH = 3,
    SET_PIN = 5,
    SET_PASSKEY = 6,
    SET_CONNECTION = 7,
    SET_BRICKNAME = 8,
    SET_MOVEUP = 9,
    SET_MOVEDOWN = 10,
    SET_ENCRYPT = 11,
    SET_SSID = 12,
}

impl ComSetSubcode {
    pub fn from_u8(i: u8) -> Result<ComSetSubcode, &'static str> {
        match i {
            1 => Ok(ComSetSubcode::SET_ON_OFF),
            2 => Ok(ComSetSubcode::SET_VISIBLE),
            3 => Ok(ComSetSubcode::SET_SEARCH),
            5 => Ok(ComSetSubcode::SET_PIN),
            6 => Ok(ComSetSubcode::SET_PASSKEY),
            7 => Ok(ComSetSubcode::SET_CONNECTION),
            8 => Ok(ComSetSubcode::SET_BRICKNAME),
            9 => Ok(ComSetSubcode::SET_MOVEUP),
            10 => Ok(ComSetSubcode::SET_MOVEDOWN),
            11 => Ok(ComSetSubcode::SET_ENCRYPT),
            12 => Ok(ComSetSubcode::SET_SSID),
            _ => Err("Invalid enum value for ComSetSubcode")
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            ComSetSubcode::SET_ON_OFF => "SET_ON_OFF",
            ComSetSubcode::SET_VISIBLE => "SET_VISIBLE",
            ComSetSubcode::SET_SEARCH => "SET_SEARCH",
            ComSetSubcode::SET_PIN => "SET_PIN",
            ComSetSubcode::SET_PASSKEY => "SET_PASSKEY",
            ComSetSubcode::SET_CONNECTION => "SET_CONNECTION",
            ComSetSubcode::SET_BRICKNAME => "SET_BRICKNAME",
            ComSetSubcode::SET_MOVEUP => "SET_MOVEUP",
            ComSetSubcode::SET_MOVEDOWN => "SET_MOVEDOWN",
            ComSetSubcode::SET_ENCRYPT => "SET_ENCRYPT",
            ComSetSubcode::SET_SSID => "SET_SSID",
        }
    }

    pub fn to_u8(&self) -> u8 {
        *self as u8
    }
}

#[derive(Copy, Clone)]
pub enum Opcode {
    Error,
    Nop,
    ProgramStop,
    ProgramStart,
    ObjectStop,
    ObjectStart,
    ObjectTrig,
    ObjectWait,
    Return,
    Call,
    ObjectEnd,
    Sleep,
    ProgramInfo(ProgramInfoSubcode),
    Label,
    Probe,
    Do,
    Add8,
    Add16,
    Add32,
    AddF,
    Sub8,
    Sub16,
    Sub32,
    SubF,
    Mul8,
    Mul16,
    Mul32,
    MulF,
    Div8,
    Div16,
    Div32,
    DivF,
    Or8,
    Or16,
    Or32,
    And8,
    And16,
    And32,
    Xor8,
    Xor16,
    Xor32,
    Rl8,
    Rl16,
    Rl32,
    InitBytes,
    Move8_8,
    Move8_16,
    Move8_32,
    Move8_F,
    Move16_8,
    Move16_16,
    Move16_32,
    Move16_F,
    Move32_8,
    Move32_16,
    Move32_32,
    Move32_F,
    MoveF_8,
    MoveF_16,
    MoveF_32,
    MoveF_F,
    Jr,
    JrFalse,
    JrTrue,
    JrNan,
    CpLt8,
    CpLt16,
    CpLt32,
    CpLtF,
    CpGt8,
    CpGt16,
    CpGt32,
    CpGtF,
    CpEq8,
    CpEq16,
    CpEq32,
    CpEqF,
    CpNeq8,
    CpNeq16,
    CpNeq32,
    CpNeqF,
    CpLteq8,
    CpLteq16,
    CpLteq32,
    CpLteqF,
    CpGteq8,
    CpGteq16,
    CpGteq32,
    CpGteqF,
    Select8,
    Select16,
    Select32,
    SelectF,
    System,
    PortCnvOutput,
    PortCnvInput,
    NoteToFreq,
    JrLt8,
    JrLt16,
    JrLt32,
    JrLtF,
    JrGt8,
    JrGt16,
    JrGt32,
    JrGtF,
    JrEq8,
    JrEq16,
    JrEq32,
    JrEqF,
    JrNeq8,
    JrNeq16,
    JrNeq32,
    JrNeqF,
    JrLteq8,
    JrLteq16,
    JrLteq32,
    JrLteqF,
    JrGteq8,
    JrGteq16,
    JrGteq32,
    JrGteqF,
    Info(InfoSubcode),
    String(StringSubcode),
    MemoryWrite,
    MemoryRead,
    UiFlush,
    UiRead(UiReadSubcode),
    UiWrite(UiWriteSubcode),
    UiButton(UiButtonSubcode),
    UiDraw(UiDrawSubcode),
    Bp0,
    Bp1,
    Bp2,
    Bp3,
    BpSet,
    Math(MathSubcode),
    Random,
    KeepAlive,
    ComRead(ComReadSubcode),
    ComWrite(ComWriteSubcode),
    Sound(SoundSubcode),
    SoundTest,
    SoundReady,
    InputDeviceList,
    InputDevice(InputDeviceSubcode),
    InputRead,
    InputTest,
    InputReady,
    InputReadSi,
    InputReadExt,
    InputWrite,
    OutputSetType,
    OutputReset,
    OutputStop,
    OutputPower,
    OutputSpeed,
    OutputStart,
    OutputPolarity,
    OutputRead,
    OutputTest,
    OutputReady,
    OutputStepPower,
    OutputTimePower,
    OutputStepSpeed,
    OutputTimeSpeed,
    OutputStepSync,
    OutputTimeSync,
    OutputClrCount,
    OutputGetCount,
    OutputPrgStop,
    File(FileSubcode),
    Array(ArraySubcode),
    ArrayWrite,
    ArrayRead,
    ArrayAppend,
    MemoryUsage,
    Filename(FilenameSubcode),
    Read8,
    Read16,
    Read32,
    Readf,
    Write8,
    Write16,
    Write32,
    Writef,
    ComReady,
    ComGet(ComGetSubcode),
    ComSet(ComSetSubcode),
    ComTest,
    MailboxOpen,
    MailboxWrite,
    MailboxRead,
    MailboxTest,
    MailboxReady,
    MailboxClose,
}

impl Opcode {
    pub fn u8_has_subcode(op: u8) -> bool {
        match op {
            0x0C => true,
            0x7C => true,
            0x7D => true,
            0x81 => true,
            0x82 => true,
            0x83 => true,
            0x84 => true,
            0x8D => true,
            0x91 => true,
            0x92 => true,
            0x94 => true,
            0x99 => true,
            0xC0 => true,
            0xC1 => true,
            0xC6 => true,
            0xD3 => true,
            0xD4 => true,
            _ => false,
        }
    }

    pub fn has_subcode(&self) -> bool {
        Opcode::u8_has_subcode(self.to_u8())
    }

    pub fn from_u8(op: u8, subcode: u8) -> Result<Opcode, &'static str> {
        match op {
            0x00 => Ok(Opcode::Error),
            0x01 => Ok(Opcode::Nop),
            0x02 => Ok(Opcode::ProgramStop),
            0x03 => Ok(Opcode::ProgramStart),
            0x04 => Ok(Opcode::ObjectStop),
            0x05 => Ok(Opcode::ObjectStart),
            0x06 => Ok(Opcode::ObjectTrig),
            0x07 => Ok(Opcode::ObjectWait),
            0x08 => Ok(Opcode::Return),
            0x09 => Ok(Opcode::Call),
            0x0A => Ok(Opcode::ObjectEnd),
            0x0B => Ok(Opcode::Sleep),
            0x0C => Ok(Opcode::ProgramInfo(ProgramInfoSubcode::from_u8(subcode)?)),
            0x0D => Ok(Opcode::Label),
            0x0E => Ok(Opcode::Probe),
            0x0F => Ok(Opcode::Do),
            0x10 => Ok(Opcode::Add8),
            0x11 => Ok(Opcode::Add16),
            0x12 => Ok(Opcode::Add32),
            0x13 => Ok(Opcode::AddF),
            0x14 => Ok(Opcode::Sub8),
            0x15 => Ok(Opcode::Sub16),
            0x16 => Ok(Opcode::Sub32),
            0x17 => Ok(Opcode::SubF),
            0x18 => Ok(Opcode::Mul8),
            0x19 => Ok(Opcode::Mul16),
            0x1A => Ok(Opcode::Mul32),
            0x1B => Ok(Opcode::MulF),
            0x1C => Ok(Opcode::Div8),
            0x1D => Ok(Opcode::Div16),
            0x1E => Ok(Opcode::Div32),
            0x1F => Ok(Opcode::DivF),
            0x20 => Ok(Opcode::Or8),
            0x21 => Ok(Opcode::Or16),
            0x22 => Ok(Opcode::Or32),
            0x24 => Ok(Opcode::And8),
            0x25 => Ok(Opcode::And16),
            0x26 => Ok(Opcode::And32),
            0x28 => Ok(Opcode::Xor8),
            0x29 => Ok(Opcode::Xor16),
            0x2A => Ok(Opcode::Xor32),
            0x2C => Ok(Opcode::Rl8),
            0x2D => Ok(Opcode::Rl16),
            0x2E => Ok(Opcode::Rl32),
            0x2F => Ok(Opcode::InitBytes),
            0x30 => Ok(Opcode::Move8_8),
            0x31 => Ok(Opcode::Move8_16),
            0x32 => Ok(Opcode::Move8_32),
            0x33 => Ok(Opcode::Move8_F),
            0x34 => Ok(Opcode::Move16_8),
            0x35 => Ok(Opcode::Move16_16),
            0x36 => Ok(Opcode::Move16_32),
            0x37 => Ok(Opcode::Move16_F),
            0x38 => Ok(Opcode::Move32_8),
            0x39 => Ok(Opcode::Move32_16),
            0x3A => Ok(Opcode::Move32_32),
            0x3B => Ok(Opcode::Move32_F),
            0x3C => Ok(Opcode::MoveF_8),
            0x3D => Ok(Opcode::MoveF_16),
            0x3E => Ok(Opcode::MoveF_32),
            0x3F => Ok(Opcode::MoveF_F),
            0x40 => Ok(Opcode::Jr),
            0x41 => Ok(Opcode::JrFalse),
            0x42 => Ok(Opcode::JrTrue),
            0x43 => Ok(Opcode::JrNan),
            0x44 => Ok(Opcode::CpLt8),
            0x45 => Ok(Opcode::CpLt16),
            0x46 => Ok(Opcode::CpLt32),
            0x47 => Ok(Opcode::CpLtF),
            0x48 => Ok(Opcode::CpGt8),
            0x49 => Ok(Opcode::CpGt16),
            0x4A => Ok(Opcode::CpGt32),
            0x4B => Ok(Opcode::CpGtF),
            0x4C => Ok(Opcode::CpEq8),
            0x4D => Ok(Opcode::CpEq16),
            0x4E => Ok(Opcode::CpEq32),
            0x4F => Ok(Opcode::CpEqF),
            0x50 => Ok(Opcode::CpNeq8),
            0x51 => Ok(Opcode::CpNeq16),
            0x52 => Ok(Opcode::CpNeq32),
            0x53 => Ok(Opcode::CpNeqF),
            0x54 => Ok(Opcode::CpLteq8),
            0x55 => Ok(Opcode::CpLteq16),
            0x56 => Ok(Opcode::CpLteq32),
            0x57 => Ok(Opcode::CpLteqF),
            0x58 => Ok(Opcode::CpGteq8),
            0x59 => Ok(Opcode::CpGteq16),
            0x5A => Ok(Opcode::CpGteq32),
            0x5B => Ok(Opcode::CpGteqF),
            0x5C => Ok(Opcode::Select8),
            0x5D => Ok(Opcode::Select16),
            0x5E => Ok(Opcode::Select32),
            0x5F => Ok(Opcode::SelectF),
            0x60 => Ok(Opcode::System),
            0x61 => Ok(Opcode::PortCnvOutput),
            0x62 => Ok(Opcode::PortCnvInput),
            0x63 => Ok(Opcode::NoteToFreq),
            0x64 => Ok(Opcode::JrLt8),
            0x65 => Ok(Opcode::JrLt16),
            0x66 => Ok(Opcode::JrLt32),
            0x67 => Ok(Opcode::JrLtF),
            0x68 => Ok(Opcode::JrGt8),
            0x69 => Ok(Opcode::JrGt16),
            0x6A => Ok(Opcode::JrGt32),
            0x6B => Ok(Opcode::JrGtF),
            0x6C => Ok(Opcode::JrEq8),
            0x6D => Ok(Opcode::JrEq16),
            0x6E => Ok(Opcode::JrEq32),
            0x6F => Ok(Opcode::JrEqF),
            0x70 => Ok(Opcode::JrNeq8),
            0x71 => Ok(Opcode::JrNeq16),
            0x72 => Ok(Opcode::JrNeq32),
            0x73 => Ok(Opcode::JrNeqF),
            0x74 => Ok(Opcode::JrLteq8),
            0x75 => Ok(Opcode::JrLteq16),
            0x76 => Ok(Opcode::JrLteq32),
            0x77 => Ok(Opcode::JrLteqF),
            0x78 => Ok(Opcode::JrGteq8),
            0x79 => Ok(Opcode::JrGteq16),
            0x7A => Ok(Opcode::JrGteq32),
            0x7B => Ok(Opcode::JrGteqF),
            0x7C => Ok(Opcode::Info(InfoSubcode::from_u8(subcode)?)),
            0x7D => Ok(Opcode::String(StringSubcode::from_u8(subcode)?)),
            0x7E => Ok(Opcode::MemoryWrite),
            0x7F => Ok(Opcode::MemoryRead),
            0x80 => Ok(Opcode::UiFlush),
            0x81 => Ok(Opcode::UiRead(UiReadSubcode::from_u8(subcode)?)),
            0x82 => Ok(Opcode::UiWrite(UiWriteSubcode::from_u8(subcode)?)),
            0x83 => Ok(Opcode::UiButton(UiButtonSubcode::from_u8(subcode)?)),
            0x84 => Ok(Opcode::UiDraw(UiDrawSubcode::from_u8(subcode)?)),
            0x88 => Ok(Opcode::Bp0),
            0x89 => Ok(Opcode::Bp1),
            0x8A => Ok(Opcode::Bp2),
            0x8B => Ok(Opcode::Bp3),
            0x8C => Ok(Opcode::BpSet),
            0x8D => Ok(Opcode::Math(MathSubcode::from_u8(subcode)?)),
            0x8E => Ok(Opcode::Random),
            0x90 => Ok(Opcode::KeepAlive),
            0x91 => Ok(Opcode::ComRead(ComReadSubcode::from_u8(subcode)?)),
            0x92 => Ok(Opcode::ComWrite(ComWriteSubcode::from_u8(subcode)?)),
            0x94 => Ok(Opcode::Sound(SoundSubcode::from_u8(subcode)?)),
            0x95 => Ok(Opcode::SoundTest),
            0x96 => Ok(Opcode::SoundReady),
            0x98 => Ok(Opcode::InputDeviceList),
            0x99 => Ok(Opcode::InputDevice(InputDeviceSubcode::from_u8(subcode)?)),
            0x9A => Ok(Opcode::InputRead),
            0x9B => Ok(Opcode::InputTest),
            0x9C => Ok(Opcode::InputReady),
            0x9D => Ok(Opcode::InputReadSi),
            0x9E => Ok(Opcode::InputReadExt),
            0x9F => Ok(Opcode::InputWrite),
            0xA1 => Ok(Opcode::OutputSetType),
            0xA2 => Ok(Opcode::OutputReset),
            0xA3 => Ok(Opcode::OutputStop),
            0xA4 => Ok(Opcode::OutputPower),
            0xA5 => Ok(Opcode::OutputSpeed),
            0xA6 => Ok(Opcode::OutputStart),
            0xA7 => Ok(Opcode::OutputPolarity),
            0xA8 => Ok(Opcode::OutputRead),
            0xA9 => Ok(Opcode::OutputTest),
            0xAA => Ok(Opcode::OutputReady),
            0xAC => Ok(Opcode::OutputStepPower),
            0xAD => Ok(Opcode::OutputTimePower),
            0xAE => Ok(Opcode::OutputStepSpeed),
            0xAF => Ok(Opcode::OutputTimeSpeed),
            0xB0 => Ok(Opcode::OutputStepSync),
            0xB1 => Ok(Opcode::OutputTimeSync),
            0xB2 => Ok(Opcode::OutputClrCount),
            0xB3 => Ok(Opcode::OutputGetCount),
            0xB4 => Ok(Opcode::OutputPrgStop),
            0xC0 => Ok(Opcode::File(FileSubcode::from_u8(subcode)?)),
            0xC1 => Ok(Opcode::Array(ArraySubcode::from_u8(subcode)?)),
            0xC2 => Ok(Opcode::ArrayWrite),
            0xC3 => Ok(Opcode::ArrayRead),
            0xC4 => Ok(Opcode::ArrayAppend),
            0xC5 => Ok(Opcode::MemoryUsage),
            0xC6 => Ok(Opcode::Filename(FilenameSubcode::from_u8(subcode)?)),
            0xC8 => Ok(Opcode::Read8),
            0xC9 => Ok(Opcode::Read16),
            0xCA => Ok(Opcode::Read32),
            0xCB => Ok(Opcode::Readf),
            0xCC => Ok(Opcode::Write8),
            0xCD => Ok(Opcode::Write16),
            0xCE => Ok(Opcode::Write32),
            0xCF => Ok(Opcode::Writef),
            0xD0 => Ok(Opcode::ComReady),
            0xD3 => Ok(Opcode::ComGet(ComGetSubcode::from_u8(subcode)?)),
            0xD4 => Ok(Opcode::ComSet(ComSetSubcode::from_u8(subcode)?)),
            0xD5 => Ok(Opcode::ComTest),
            0xD8 => Ok(Opcode::MailboxOpen),
            0xD9 => Ok(Opcode::MailboxWrite),
            0xDA => Ok(Opcode::MailboxRead),
            0xDB => Ok(Opcode::MailboxTest),
            0xDC => Ok(Opcode::MailboxReady),
            0xDD => Ok(Opcode::MailboxClose),
            _ => Err("Invalid enum value for Opcode")
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Opcode::Error => "Error",
            Opcode::Nop => "Nop",
            Opcode::ProgramStop => "ProgramStop",
            Opcode::ProgramStart => "ProgramStart",
            Opcode::ObjectStop => "ObjectStop",
            Opcode::ObjectStart => "ObjectStart",
            Opcode::ObjectTrig => "ObjectTrig",
            Opcode::ObjectWait => "ObjectWait",
            Opcode::Return => "Return",
            Opcode::Call => "Call",
            Opcode::ObjectEnd => "ObjectEnd",
            Opcode::Sleep => "Sleep",
            Opcode::ProgramInfo(subcode) => match subcode {
                ProgramInfoSubcode::OBJ_STOP => "ProgramInfo(OBJ_STOP)",
                ProgramInfoSubcode::OBJ_START => "ProgramInfo(OBJ_START)",
                ProgramInfoSubcode::GET_STATUS => "ProgramInfo(GET_STATUS)",
                ProgramInfoSubcode::GET_SPEED => "ProgramInfo(GET_SPEED)",
                ProgramInfoSubcode::GET_PRGRESULT => "ProgramInfo(GET_PRGRESULT)",
            },
            Opcode::Label => "Label",
            Opcode::Probe => "Probe",
            Opcode::Do => "Do",
            Opcode::Add8 => "Add8",
            Opcode::Add16 => "Add16",
            Opcode::Add32 => "Add32",
            Opcode::AddF => "AddF",
            Opcode::Sub8 => "Sub8",
            Opcode::Sub16 => "Sub16",
            Opcode::Sub32 => "Sub32",
            Opcode::SubF => "SubF",
            Opcode::Mul8 => "Mul8",
            Opcode::Mul16 => "Mul16",
            Opcode::Mul32 => "Mul32",
            Opcode::MulF => "MulF",
            Opcode::Div8 => "Div8",
            Opcode::Div16 => "Div16",
            Opcode::Div32 => "Div32",
            Opcode::DivF => "DivF",
            Opcode::Or8 => "Or8",
            Opcode::Or16 => "Or16",
            Opcode::Or32 => "Or32",
            Opcode::And8 => "And8",
            Opcode::And16 => "And16",
            Opcode::And32 => "And32",
            Opcode::Xor8 => "Xor8",
            Opcode::Xor16 => "Xor16",
            Opcode::Xor32 => "Xor32",
            Opcode::Rl8 => "Rl8",
            Opcode::Rl16 => "Rl16",
            Opcode::Rl32 => "Rl32",
            Opcode::InitBytes => "InitBytes",
            Opcode::Move8_8 => "Move8_8",
            Opcode::Move8_16 => "Move8_16",
            Opcode::Move8_32 => "Move8_32",
            Opcode::Move8_F => "Move8_F",
            Opcode::Move16_8 => "Move16_8",
            Opcode::Move16_16 => "Move16_16",
            Opcode::Move16_32 => "Move16_32",
            Opcode::Move16_F => "Move16_F",
            Opcode::Move32_8 => "Move32_8",
            Opcode::Move32_16 => "Move32_16",
            Opcode::Move32_32 => "Move32_32",
            Opcode::Move32_F => "Move32_F",
            Opcode::MoveF_8 => "MoveF_8",
            Opcode::MoveF_16 => "MoveF_16",
            Opcode::MoveF_32 => "MoveF_32",
            Opcode::MoveF_F => "MoveF_F",
            Opcode::Jr => "Jr",
            Opcode::JrFalse => "JrFalse",
            Opcode::JrTrue => "JrTrue",
            Opcode::JrNan => "JrNan",
            Opcode::CpLt8 => "CpLt8",
            Opcode::CpLt16 => "CpLt16",
            Opcode::CpLt32 => "CpLt32",
            Opcode::CpLtF => "CpLtF",
            Opcode::CpGt8 => "CpGt8",
            Opcode::CpGt16 => "CpGt16",
            Opcode::CpGt32 => "CpGt32",
            Opcode::CpGtF => "CpGtF",
            Opcode::CpEq8 => "CpEq8",
            Opcode::CpEq16 => "CpEq16",
            Opcode::CpEq32 => "CpEq32",
            Opcode::CpEqF => "CpEqF",
            Opcode::CpNeq8 => "CpNeq8",
            Opcode::CpNeq16 => "CpNeq16",
            Opcode::CpNeq32 => "CpNeq32",
            Opcode::CpNeqF => "CpNeqF",
            Opcode::CpLteq8 => "CpLteq8",
            Opcode::CpLteq16 => "CpLteq16",
            Opcode::CpLteq32 => "CpLteq32",
            Opcode::CpLteqF => "CpLteqF",
            Opcode::CpGteq8 => "CpGteq8",
            Opcode::CpGteq16 => "CpGteq16",
            Opcode::CpGteq32 => "CpGteq32",
            Opcode::CpGteqF => "CpGteqF",
            Opcode::Select8 => "Select8",
            Opcode::Select16 => "Select16",
            Opcode::Select32 => "Select32",
            Opcode::SelectF => "SelectF",
            Opcode::System => "System",
            Opcode::PortCnvOutput => "PortCnvOutput",
            Opcode::PortCnvInput => "PortCnvInput",
            Opcode::NoteToFreq => "NoteToFreq",
            Opcode::JrLt8 => "JrLt8",
            Opcode::JrLt16 => "JrLt16",
            Opcode::JrLt32 => "JrLt32",
            Opcode::JrLtF => "JrLtF",
            Opcode::JrGt8 => "JrGt8",
            Opcode::JrGt16 => "JrGt16",
            Opcode::JrGt32 => "JrGt32",
            Opcode::JrGtF => "JrGtF",
            Opcode::JrEq8 => "JrEq8",
            Opcode::JrEq16 => "JrEq16",
            Opcode::JrEq32 => "JrEq32",
            Opcode::JrEqF => "JrEqF",
            Opcode::JrNeq8 => "JrNeq8",
            Opcode::JrNeq16 => "JrNeq16",
            Opcode::JrNeq32 => "JrNeq32",
            Opcode::JrNeqF => "JrNeqF",
            Opcode::JrLteq8 => "JrLteq8",
            Opcode::JrLteq16 => "JrLteq16",
            Opcode::JrLteq32 => "JrLteq32",
            Opcode::JrLteqF => "JrLteqF",
            Opcode::JrGteq8 => "JrGteq8",
            Opcode::JrGteq16 => "JrGteq16",
            Opcode::JrGteq32 => "JrGteq32",
            Opcode::JrGteqF => "JrGteqF",
            Opcode::Info(subcode) => match subcode {
                InfoSubcode::SET_ERROR => "Info(SET_ERROR)",
                InfoSubcode::GET_ERROR => "Info(GET_ERROR)",
                InfoSubcode::ERRORTEXT => "Info(ERRORTEXT)",
                InfoSubcode::GET_VOLUME => "Info(GET_VOLUME)",
                InfoSubcode::SET_VOLUME => "Info(SET_VOLUME)",
                InfoSubcode::GET_MINUTES => "Info(GET_MINUTES)",
                InfoSubcode::SET_MINUTES => "Info(SET_MINUTES)",
            },
            Opcode::String(subcode) => match subcode {
                StringSubcode::GET_SIZE => "String(GET_SIZE)",
                StringSubcode::ADD => "String(ADD)",
                StringSubcode::COMPARE => "String(COMPARE)",
                StringSubcode::DUPLICATE => "String(DUPLICATE)",
                StringSubcode::VALUE_TO_STRING => "String(VALUE_TO_STRING)",
                StringSubcode::STRING_TO_VALUE => "String(STRING_TO_VALUE)",
                StringSubcode::STRIP => "String(STRIP)",
                StringSubcode::NUMBER_TO_STRING => "String(NUMBER_TO_STRING)",
                StringSubcode::SUB => "String(SUB)",
                StringSubcode::VALUE_FORMATTED => "String(VALUE_FORMATTED)",
                StringSubcode::NUMBER_FORMATTED => "String(NUMBER_FORMATTED)",
            },
            Opcode::MemoryWrite => "MemoryWrite",
            Opcode::MemoryRead => "MemoryRead",
            Opcode::UiFlush => "UiFlush",
            Opcode::UiRead(subcode) => match subcode {
                UiReadSubcode::GET_VBATT => "UiRead(GET_VBATT)",
                UiReadSubcode::GET_IBATT => "UiRead(GET_IBATT)",
                UiReadSubcode::GET_OS_VERS => "UiRead(GET_OS_VERS)",
                UiReadSubcode::GET_EVENT => "UiRead(GET_EVENT)",
                UiReadSubcode::GET_TBATT => "UiRead(GET_TBATT)",
                UiReadSubcode::GET_IMOTOR => "UiRead(GET_IMOTOR)",
                UiReadSubcode::GET_STRING => "UiRead(GET_STRING)",
                UiReadSubcode::GET_HW_VERS => "UiRead(GET_HW_VERS)",
                UiReadSubcode::GET_FW_VERS => "UiRead(GET_FW_VERS)",
                UiReadSubcode::GET_FW_BUILD => "UiRead(GET_FW_BUILD)",
                UiReadSubcode::GET_OS_BUILD => "UiRead(GET_OS_BUILD)",
                UiReadSubcode::GET_ADDRESS => "UiRead(GET_ADDRESS)",
                UiReadSubcode::GET_CODE => "UiRead(GET_CODE)",
                UiReadSubcode::KEY => "UiRead(KEY)",
                UiReadSubcode::GET_SHUTDOWN => "UiRead(GET_SHUTDOWN)",
                UiReadSubcode::GET_WARNING => "UiRead(GET_WARNING)",
                UiReadSubcode::GET_LBATT => "UiRead(GET_LBATT)",
                UiReadSubcode::TEXTBOX_READ => "UiRead(TEXTBOX_READ)",
                UiReadSubcode::GET_VERSION => "UiRead(GET_VERSION)",
                UiReadSubcode::GET_IP => "UiRead(GET_IP)",
                UiReadSubcode::GET_SDCARD => "UiRead(GET_SDCARD)",
                UiReadSubcode::GET_USBSTICK => "UiRead(GET_USBSTICK)",
            },
            Opcode::UiWrite(subcode) => match subcode {
                UiWriteSubcode::WRITE_FLUSH => "UiWrite(WRITE_FLUSH)",
                UiWriteSubcode::FLOATVALUE => "UiWrite(FLOATVALUE)",
                UiWriteSubcode::PUT_STRING => "UiWrite(PUT_STRING)",
                UiWriteSubcode::VALUE8 => "UiWrite(VALUE8)",
                UiWriteSubcode::VALUE16 => "UiWrite(VALUE16)",
                UiWriteSubcode::VALUE32 => "UiWrite(VALUE32)",
                UiWriteSubcode::VALUEF => "UiWrite(VALUEF)",
                UiWriteSubcode::DOWNLOAD_END => "UiWrite(DOWNLOAD_END)",
                UiWriteSubcode::SCREEN_BLOCK => "UiWrite(SCREEN_BLOCK)",
                UiWriteSubcode::TEXTBOX_APPEND => "UiWrite(TEXTBOX_APPEND)",
                UiWriteSubcode::SET_BUSY => "UiWrite(SET_BUSY)",
                UiWriteSubcode::SET_TESTPIN => "UiWrite(SET_TESTPIN)",
                UiWriteSubcode::INIT_RUN => "UiWrite(INIT_RUN)",
                UiWriteSubcode::LED => "UiWrite(LED)",
                UiWriteSubcode::POWER => "UiWrite(POWER)",
                UiWriteSubcode::GRAPH_SAMPLE => "UiWrite(GRAPH_SAMPLE)",
                UiWriteSubcode::TERMINAL => "UiWrite(TERMINAL)",
            },
            Opcode::UiButton(subcode) => match subcode {
                UiButtonSubcode::SHORTPRESS => "UiButton(SHORTPRESS)",
                UiButtonSubcode::LONGPRESS => "UiButton(LONGPRESS)",
                UiButtonSubcode::WAIT_FOR_PRESS => "UiButton(WAIT_FOR_PRESS)",
                UiButtonSubcode::FLUSH => "UiButton(FLUSH)",
                UiButtonSubcode::PRESS => "UiButton(PRESS)",
                UiButtonSubcode::RELEASE => "UiButton(RELEASE)",
                UiButtonSubcode::GET_HORZ => "UiButton(GET_HORZ)",
                UiButtonSubcode::GET_VERT => "UiButton(GET_VERT)",
                UiButtonSubcode::PRESSED => "UiButton(PRESSED)",
                UiButtonSubcode::SET_BACK_BLOCK => "UiButton(SET_BACK_BLOCK)",
                UiButtonSubcode::GET_BACK_BLOCK => "UiButton(GET_BACK_BLOCK)",
                UiButtonSubcode::TESTSHORTPRESS => "UiButton(TESTSHORTPRESS)",
                UiButtonSubcode::TESTLONGPRESS => "UiButton(TESTLONGPRESS)",
                UiButtonSubcode::GET_BUMBED => "UiButton(GET_BUMBED)",
                UiButtonSubcode::GET_CLICK => "UiButton(GET_CLICK)",
            },
            Opcode::UiDraw(subcode) => match subcode {
                UiDrawSubcode::UPDATE => "UiDraw(UPDATE)",
                UiDrawSubcode::PIXEL => "UiDraw(PIXEL)",
                UiDrawSubcode::LINE => "UiDraw(LINE)",
                UiDrawSubcode::CIRCLE => "UiDraw(CIRCLE)",
                UiDrawSubcode::TEXT => "UiDraw(TEXT)",
                UiDrawSubcode::ICON => "UiDraw(ICON)",
                UiDrawSubcode::PICTURE => "UiDraw(PICTURE)",
                UiDrawSubcode::VALUE => "UiDraw(VALUE)",
                UiDrawSubcode::FILLRECT => "UiDraw(FILLRECT)",
                UiDrawSubcode::RECT => "UiDraw(RECT)",
                UiDrawSubcode::NOTIFICATION => "UiDraw(NOTIFICATION)",
                UiDrawSubcode::QUESTION => "UiDraw(QUESTION)",
                UiDrawSubcode::KEYBOARD => "UiDraw(KEYBOARD)",
                UiDrawSubcode::BROWSE => "UiDraw(BROWSE)",
                UiDrawSubcode::VERTBAR => "UiDraw(VERTBAR)",
                UiDrawSubcode::INVERSERECT => "UiDraw(INVERSERECT)",
                UiDrawSubcode::SELECT_FONT => "UiDraw(SELECT_FONT)",
                UiDrawSubcode::TOPLINE => "UiDraw(TOPLINE)",
                UiDrawSubcode::FILLWINDOW => "UiDraw(FILLWINDOW)",
                UiDrawSubcode::DOTLINE => "UiDraw(DOTLINE)",
                UiDrawSubcode::VIEW_VALUE => "UiDraw(VIEW_VALUE)",
                UiDrawSubcode::VIEW_UNIT => "UiDraw(VIEW_UNIT)",
                UiDrawSubcode::FILLCIRCLE => "UiDraw(FILLCIRCLE)",
                UiDrawSubcode::STORE => "UiDraw(STORE)",
                UiDrawSubcode::RESTORE => "UiDraw(RESTORE)",
                UiDrawSubcode::ICON_QUESTION => "UiDraw(ICON_QUESTION)",
                UiDrawSubcode::BMPFILE => "UiDraw(BMPFILE)",
                UiDrawSubcode::GRAPH_SETUP => "UiDraw(GRAPH_SETUP)",
                UiDrawSubcode::GRAPH_DRAW => "UiDraw(GRAPH_DRAW)",
                UiDrawSubcode::TEXTBOX => "UiDraw(TEXTBOX)",
            },
            Opcode::Bp0 => "Bp0",
            Opcode::Bp1 => "Bp1",
            Opcode::Bp2 => "Bp2",
            Opcode::Bp3 => "Bp3",
            Opcode::BpSet => "BpSet",
            Opcode::Math(subcode) => match subcode {
                MathSubcode::EXP => "Math(EXP)",
                MathSubcode::MOD => "Math(MOD)",
                MathSubcode::FLOOR => "Math(FLOOR)",
                MathSubcode::CEIL => "Math(CEIL)",
                MathSubcode::ROUND => "Math(ROUND)",
                MathSubcode::ABS => "Math(ABS)",
                MathSubcode::NEGATE => "Math(NEGATE)",
                MathSubcode::SQRT => "Math(SQRT)",
                MathSubcode::LOG => "Math(LOG)",
                MathSubcode::LN => "Math(LN)",
                MathSubcode::SIN => "Math(SIN)",
                MathSubcode::COS => "Math(COS)",
                MathSubcode::TAN => "Math(TAN)",
                MathSubcode::ASIN => "Math(ASIN)",
                MathSubcode::ACOS => "Math(ACOS)",
                MathSubcode::ATAN => "Math(ATAN)",
                MathSubcode::MOD8 => "Math(MOD8)",
                MathSubcode::MOD16 => "Math(MOD16)",
                MathSubcode::MOD32 => "Math(MOD32)",
                MathSubcode::POW => "Math(POW)",
                MathSubcode::TRUNC => "Math(TRUNC)",
            },
            Opcode::Random => "Random",
            Opcode::KeepAlive => "KeepAlive",
            Opcode::ComRead(subcode) => match subcode {
                ComReadSubcode::COMMAND => "ComRead(COMMAND)",
            },
            Opcode::ComWrite(subcode) => match subcode {
                ComWriteSubcode::REPLY => "ComWrite(REPLY)",
            },
            Opcode::Sound(subcode) => match subcode {
                SoundSubcode::BREAK => "Sound(BREAK)",
                SoundSubcode::TONE => "Sound(TONE)",
                SoundSubcode::PLAY => "Sound(PLAY)",
                SoundSubcode::REPEAT => "Sound(REPEAT)",
            },
            Opcode::SoundTest => "SoundTest",
            Opcode::SoundReady => "SoundReady",
            Opcode::InputDeviceList => "InputDeviceList",
            Opcode::InputDevice(subcode) => match subcode {
                InputDeviceSubcode::GET_FORMAT => "InputDevice(GET_FORMAT)",
                InputDeviceSubcode::CAL_MINMAX => "InputDevice(CAL_MINMAX)",
                InputDeviceSubcode::CAL_DEFAULT => "InputDevice(CAL_DEFAULT)",
                InputDeviceSubcode::GET_TYPEMODE => "InputDevice(GET_TYPEMODE)",
                InputDeviceSubcode::GET_SYMBOL => "InputDevice(GET_SYMBOL)",
                InputDeviceSubcode::CAL_MIN => "InputDevice(CAL_MIN)",
                InputDeviceSubcode::CAL_MAX => "InputDevice(CAL_MAX)",
                InputDeviceSubcode::SETUP => "InputDevice(SETUP)",
                InputDeviceSubcode::CLR_ALL => "InputDevice(CLR_ALL)",
                InputDeviceSubcode::GET_RAW => "InputDevice(GET_RAW)",
                InputDeviceSubcode::GET_CONNECTION => "InputDevice(GET_CONNECTION)",
                InputDeviceSubcode::STOP_ALL => "InputDevice(STOP_ALL)",
                InputDeviceSubcode::GET_NAME => "InputDevice(GET_NAME)",
                InputDeviceSubcode::GET_MODENAME => "InputDevice(GET_MODENAME)",
                InputDeviceSubcode::GET_FIGURES => "InputDevice(GET_FIGURES)",
                InputDeviceSubcode::GET_CHANGES => "InputDevice(GET_CHANGES)",
                InputDeviceSubcode::CLR_CHANGES => "InputDevice(CLR_CHANGES)",
                InputDeviceSubcode::READY_PCT => "InputDevice(READY_PCT)",
                InputDeviceSubcode::READY_RAW => "InputDevice(READY_RAW)",
                InputDeviceSubcode::READY_SI => "InputDevice(READY_SI)",
                InputDeviceSubcode::GET_MINMAX => "InputDevice(GET_MINMAX)",
                InputDeviceSubcode::GET_BUMPS => "InputDevice(GET_BUMPS)",
            },
            Opcode::InputRead => "InputRead",
            Opcode::InputTest => "InputTest",
            Opcode::InputReady => "InputReady",
            Opcode::InputReadSi => "InputReadSi",
            Opcode::InputReadExt => "InputReadExt",
            Opcode::InputWrite => "InputWrite",
            Opcode::OutputSetType => "OutputSetType",
            Opcode::OutputReset => "OutputReset",
            Opcode::OutputStop => "OutputStop",
            Opcode::OutputPower => "OutputPower",
            Opcode::OutputSpeed => "OutputSpeed",
            Opcode::OutputStart => "OutputStart",
            Opcode::OutputPolarity => "OutputPolarity",
            Opcode::OutputRead => "OutputRead",
            Opcode::OutputTest => "OutputTest",
            Opcode::OutputReady => "OutputReady",
            Opcode::OutputStepPower => "OutputStepPower",
            Opcode::OutputTimePower => "OutputTimePower",
            Opcode::OutputStepSpeed => "OutputStepSpeed",
            Opcode::OutputTimeSpeed => "OutputTimeSpeed",
            Opcode::OutputStepSync => "OutputStepSync",
            Opcode::OutputTimeSync => "OutputTimeSync",
            Opcode::OutputClrCount => "OutputClrCount",
            Opcode::OutputGetCount => "OutputGetCount",
            Opcode::OutputPrgStop => "OutputPrgStop",
            Opcode::File(subcode) => match subcode {
                FileSubcode::OPEN_APPEND => "File(OPEN_APPEND)",
                FileSubcode::OPEN_READ => "File(OPEN_READ)",
                FileSubcode::OPEN_WRITE => "File(OPEN_WRITE)",
                FileSubcode::READ_VALUE => "File(READ_VALUE)",
                FileSubcode::WRITE_VALUE => "File(WRITE_VALUE)",
                FileSubcode::READ_TEXT => "File(READ_TEXT)",
                FileSubcode::WRITE_TEXT => "File(WRITE_TEXT)",
                FileSubcode::CLOSE => "File(CLOSE)",
                FileSubcode::LOAD_IMAGE => "File(LOAD_IMAGE)",
                FileSubcode::GET_HANDLE => "File(GET_HANDLE)",
                FileSubcode::MAKE_FOLDER => "File(MAKE_FOLDER)",
                FileSubcode::GET_POOL => "File(GET_POOL)",
                FileSubcode::SET_LOG_SYNC_TIME => "File(SET_LOG_SYNC_TIME)",
                FileSubcode::GET_FOLDERS => "File(GET_FOLDERS)",
                FileSubcode::GET_LOG_SYNC_TIME => "File(GET_LOG_SYNC_TIME)",
                FileSubcode::GET_SUBFOLDER_NAME => "File(GET_SUBFOLDER_NAME)",
                FileSubcode::WRITE_LOG => "File(WRITE_LOG)",
                FileSubcode::CLOSE_LOG => "File(CLOSE_LOG)",
                FileSubcode::GET_IMAGE => "File(GET_IMAGE)",
                FileSubcode::GET_ITEM => "File(GET_ITEM)",
                FileSubcode::GET_CACHE_FILES => "File(GET_CACHE_FILES)",
                FileSubcode::PUT_CACHE_FILE => "File(PUT_CACHE_FILE)",
                FileSubcode::GET_CACHE_FILE => "File(GET_CACHE_FILE)",
                FileSubcode::DEL_CACHE_FILE => "File(DEL_CACHE_FILE)",
                FileSubcode::DEL_SUBFOLDER => "File(DEL_SUBFOLDER)",
                FileSubcode::GET_LOG_NAME => "File(GET_LOG_NAME)",
                FileSubcode::OPEN_LOG => "File(OPEN_LOG)",
                FileSubcode::READ_BYTES => "File(READ_BYTES)",
                FileSubcode::WRITE_BYTES => "File(WRITE_BYTES)",
                FileSubcode::REMOVE => "File(REMOVE)",
                FileSubcode::MOVE => "File(MOVE)",
            },
            Opcode::Array(subcode) => match subcode {
                ArraySubcode::DELETE => "Array(DELETE)",
                ArraySubcode::CREATE8 => "Array(CREATE8)",
                ArraySubcode::CREATE16 => "Array(CREATE16)",
                ArraySubcode::CREATE32 => "Array(CREATE32)",
                ArraySubcode::CREATEF => "Array(CREATEF)",
                ArraySubcode::RESIZE => "Array(RESIZE)",
                ArraySubcode::FILL => "Array(FILL)",
                ArraySubcode::COPY => "Array(COPY)",
                ArraySubcode::INIT8 => "Array(INIT8)",
                ArraySubcode::INIT16 => "Array(INIT16)",
                ArraySubcode::INIT32 => "Array(INIT32)",
                ArraySubcode::INITF => "Array(INITF)",
                ArraySubcode::SIZE => "Array(SIZE)",
                ArraySubcode::READ_CONTENT => "Array(READ_CONTENT)",
                ArraySubcode::WRITE_CONTENT => "Array(WRITE_CONTENT)",
                ArraySubcode::READ_SIZE => "Array(READ_SIZE)",
            },
            Opcode::ArrayWrite => "ArrayWrite",
            Opcode::ArrayRead => "ArrayRead",
            Opcode::ArrayAppend => "ArrayAppend",
            Opcode::MemoryUsage => "MemoryUsage",
            Opcode::Filename(subcode) => match subcode {
                FilenameSubcode::EXIST => "Filename(EXIST)",
                FilenameSubcode::TOTALSIZE => "Filename(TOTALSIZE)",
                FilenameSubcode::SPLIT => "Filename(SPLIT)",
                FilenameSubcode::MERGE => "Filename(MERGE)",
                FilenameSubcode::CHECK => "Filename(CHECK)",
                FilenameSubcode::PACK => "Filename(PACK)",
                FilenameSubcode::UNPACK => "Filename(UNPACK)",
                FilenameSubcode::GET_FOLDERNAME => "Filename(GET_FOLDERNAME)",
            },
            Opcode::Read8 => "Read8",
            Opcode::Read16 => "Read16",
            Opcode::Read32 => "Read32",
            Opcode::Readf => "Readf",
            Opcode::Write8 => "Write8",
            Opcode::Write16 => "Write16",
            Opcode::Write32 => "Write32",
            Opcode::Writef => "Writef",
            Opcode::ComReady => "ComReady",
            Opcode::ComGet(subcode) => match subcode {
                ComGetSubcode::GET_ON_OFF => "ComGet(GET_ON_OFF)",
                ComGetSubcode::GET_VISIBLE => "ComGet(GET_VISIBLE)",
                ComGetSubcode::GET_RESULT => "ComGet(GET_RESULT)",
                ComGetSubcode::GET_PIN => "ComGet(GET_PIN)",
                ComGetSubcode::SEARCH_ITEMS => "ComGet(SEARCH_ITEMS)",
                ComGetSubcode::SEARCH_ITEM => "ComGet(SEARCH_ITEM)",
                ComGetSubcode::FAVOUR_ITEMS => "ComGet(FAVOUR_ITEMS)",
                ComGetSubcode::FAVOUR_ITEM => "ComGet(FAVOUR_ITEM)",
                ComGetSubcode::GET_ID => "ComGet(GET_ID)",
                ComGetSubcode::GET_BRICKNAME => "ComGet(GET_BRICKNAME)",
                ComGetSubcode::GET_NETWORK => "ComGet(GET_NETWORK)",
                ComGetSubcode::GET_PRESENT => "ComGet(GET_PRESENT)",
                ComGetSubcode::GET_ENCRYPT => "ComGet(GET_ENCRYPT)",
                ComGetSubcode::GET_INCOMING => "ComGet(GET_INCOMING)",
            },
            Opcode::ComSet(subcode) => match subcode {
                ComSetSubcode::SET_ON_OFF => "ComSet(SET_ON_OFF)",
                ComSetSubcode::SET_VISIBLE => "ComSet(SET_VISIBLE)",
                ComSetSubcode::SET_SEARCH => "ComSet(SET_SEARCH)",
                ComSetSubcode::SET_PIN => "ComSet(SET_PIN)",
                ComSetSubcode::SET_PASSKEY => "ComSet(SET_PASSKEY)",
                ComSetSubcode::SET_CONNECTION => "ComSet(SET_CONNECTION)",
                ComSetSubcode::SET_BRICKNAME => "ComSet(SET_BRICKNAME)",
                ComSetSubcode::SET_MOVEUP => "ComSet(SET_MOVEUP)",
                ComSetSubcode::SET_MOVEDOWN => "ComSet(SET_MOVEDOWN)",
                ComSetSubcode::SET_ENCRYPT => "ComSet(SET_ENCRYPT)",
                ComSetSubcode::SET_SSID => "ComSet(SET_SSID)",
            },
            Opcode::ComTest => "ComTest",
            Opcode::MailboxOpen => "MailboxOpen",
            Opcode::MailboxWrite => "MailboxWrite",
            Opcode::MailboxRead => "MailboxRead",
            Opcode::MailboxTest => "MailboxTest",
            Opcode::MailboxReady => "MailboxReady",
            Opcode::MailboxClose => "MailboxClose",
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            Opcode::Error => 0x00,
            Opcode::Nop => 0x01,
            Opcode::ProgramStop => 0x02,
            Opcode::ProgramStart => 0x03,
            Opcode::ObjectStop => 0x04,
            Opcode::ObjectStart => 0x05,
            Opcode::ObjectTrig => 0x06,
            Opcode::ObjectWait => 0x07,
            Opcode::Return => 0x08,
            Opcode::Call => 0x09,
            Opcode::ObjectEnd => 0x0A,
            Opcode::Sleep => 0x0B,
            Opcode::ProgramInfo(_) => 0x0C,
            Opcode::Label => 0x0D,
            Opcode::Probe => 0x0E,
            Opcode::Do => 0x0F,
            Opcode::Add8 => 0x10,
            Opcode::Add16 => 0x11,
            Opcode::Add32 => 0x12,
            Opcode::AddF => 0x13,
            Opcode::Sub8 => 0x14,
            Opcode::Sub16 => 0x15,
            Opcode::Sub32 => 0x16,
            Opcode::SubF => 0x17,
            Opcode::Mul8 => 0x18,
            Opcode::Mul16 => 0x19,
            Opcode::Mul32 => 0x1A,
            Opcode::MulF => 0x1B,
            Opcode::Div8 => 0x1C,
            Opcode::Div16 => 0x1D,
            Opcode::Div32 => 0x1E,
            Opcode::DivF => 0x1F,
            Opcode::Or8 => 0x20,
            Opcode::Or16 => 0x21,
            Opcode::Or32 => 0x22,
            Opcode::And8 => 0x24,
            Opcode::And16 => 0x25,
            Opcode::And32 => 0x26,
            Opcode::Xor8 => 0x28,
            Opcode::Xor16 => 0x29,
            Opcode::Xor32 => 0x2A,
            Opcode::Rl8 => 0x2C,
            Opcode::Rl16 => 0x2D,
            Opcode::Rl32 => 0x2E,
            Opcode::InitBytes => 0x2F,
            Opcode::Move8_8 => 0x30,
            Opcode::Move8_16 => 0x31,
            Opcode::Move8_32 => 0x32,
            Opcode::Move8_F => 0x33,
            Opcode::Move16_8 => 0x34,
            Opcode::Move16_16 => 0x35,
            Opcode::Move16_32 => 0x36,
            Opcode::Move16_F => 0x37,
            Opcode::Move32_8 => 0x38,
            Opcode::Move32_16 => 0x39,
            Opcode::Move32_32 => 0x3A,
            Opcode::Move32_F => 0x3B,
            Opcode::MoveF_8 => 0x3C,
            Opcode::MoveF_16 => 0x3D,
            Opcode::MoveF_32 => 0x3E,
            Opcode::MoveF_F => 0x3F,
            Opcode::Jr => 0x40,
            Opcode::JrFalse => 0x41,
            Opcode::JrTrue => 0x42,
            Opcode::JrNan => 0x43,
            Opcode::CpLt8 => 0x44,
            Opcode::CpLt16 => 0x45,
            Opcode::CpLt32 => 0x46,
            Opcode::CpLtF => 0x47,
            Opcode::CpGt8 => 0x48,
            Opcode::CpGt16 => 0x49,
            Opcode::CpGt32 => 0x4A,
            Opcode::CpGtF => 0x4B,
            Opcode::CpEq8 => 0x4C,
            Opcode::CpEq16 => 0x4D,
            Opcode::CpEq32 => 0x4E,
            Opcode::CpEqF => 0x4F,
            Opcode::CpNeq8 => 0x50,
            Opcode::CpNeq16 => 0x51,
            Opcode::CpNeq32 => 0x52,
            Opcode::CpNeqF => 0x53,
            Opcode::CpLteq8 => 0x54,
            Opcode::CpLteq16 => 0x55,
            Opcode::CpLteq32 => 0x56,
            Opcode::CpLteqF => 0x57,
            Opcode::CpGteq8 => 0x58,
            Opcode::CpGteq16 => 0x59,
            Opcode::CpGteq32 => 0x5A,
            Opcode::CpGteqF => 0x5B,
            Opcode::Select8 => 0x5C,
            Opcode::Select16 => 0x5D,
            Opcode::Select32 => 0x5E,
            Opcode::SelectF => 0x5F,
            Opcode::System => 0x60,
            Opcode::PortCnvOutput => 0x61,
            Opcode::PortCnvInput => 0x62,
            Opcode::NoteToFreq => 0x63,
            Opcode::JrLt8 => 0x64,
            Opcode::JrLt16 => 0x65,
            Opcode::JrLt32 => 0x66,
            Opcode::JrLtF => 0x67,
            Opcode::JrGt8 => 0x68,
            Opcode::JrGt16 => 0x69,
            Opcode::JrGt32 => 0x6A,
            Opcode::JrGtF => 0x6B,
            Opcode::JrEq8 => 0x6C,
            Opcode::JrEq16 => 0x6D,
            Opcode::JrEq32 => 0x6E,
            Opcode::JrEqF => 0x6F,
            Opcode::JrNeq8 => 0x70,
            Opcode::JrNeq16 => 0x71,
            Opcode::JrNeq32 => 0x72,
            Opcode::JrNeqF => 0x73,
            Opcode::JrLteq8 => 0x74,
            Opcode::JrLteq16 => 0x75,
            Opcode::JrLteq32 => 0x76,
            Opcode::JrLteqF => 0x77,
            Opcode::JrGteq8 => 0x78,
            Opcode::JrGteq16 => 0x79,
            Opcode::JrGteq32 => 0x7A,
            Opcode::JrGteqF => 0x7B,
            Opcode::Info(_) => 0x7C,
            Opcode::String(_) => 0x7D,
            Opcode::MemoryWrite => 0x7E,
            Opcode::MemoryRead => 0x7F,
            Opcode::UiFlush => 0x80,
            Opcode::UiRead(_) => 0x81,
            Opcode::UiWrite(_) => 0x82,
            Opcode::UiButton(_) => 0x83,
            Opcode::UiDraw(_) => 0x84,
            Opcode::Bp0 => 0x88,
            Opcode::Bp1 => 0x89,
            Opcode::Bp2 => 0x8A,
            Opcode::Bp3 => 0x8B,
            Opcode::BpSet => 0x8C,
            Opcode::Math(_) => 0x8D,
            Opcode::Random => 0x8E,
            Opcode::KeepAlive => 0x90,
            Opcode::ComRead(_) => 0x91,
            Opcode::ComWrite(_) => 0x92,
            Opcode::Sound(_) => 0x94,
            Opcode::SoundTest => 0x95,
            Opcode::SoundReady => 0x96,
            Opcode::InputDeviceList => 0x98,
            Opcode::InputDevice(_) => 0x99,
            Opcode::InputRead => 0x9A,
            Opcode::InputTest => 0x9B,
            Opcode::InputReady => 0x9C,
            Opcode::InputReadSi => 0x9D,
            Opcode::InputReadExt => 0x9E,
            Opcode::InputWrite => 0x9F,
            Opcode::OutputSetType => 0xA1,
            Opcode::OutputReset => 0xA2,
            Opcode::OutputStop => 0xA3,
            Opcode::OutputPower => 0xA4,
            Opcode::OutputSpeed => 0xA5,
            Opcode::OutputStart => 0xA6,
            Opcode::OutputPolarity => 0xA7,
            Opcode::OutputRead => 0xA8,
            Opcode::OutputTest => 0xA9,
            Opcode::OutputReady => 0xAA,
            Opcode::OutputStepPower => 0xAC,
            Opcode::OutputTimePower => 0xAD,
            Opcode::OutputStepSpeed => 0xAE,
            Opcode::OutputTimeSpeed => 0xAF,
            Opcode::OutputStepSync => 0xB0,
            Opcode::OutputTimeSync => 0xB1,
            Opcode::OutputClrCount => 0xB2,
            Opcode::OutputGetCount => 0xB3,
            Opcode::OutputPrgStop => 0xB4,
            Opcode::File(_) => 0xC0,
            Opcode::Array(_) => 0xC1,
            Opcode::ArrayWrite => 0xC2,
            Opcode::ArrayRead => 0xC3,
            Opcode::ArrayAppend => 0xC4,
            Opcode::MemoryUsage => 0xC5,
            Opcode::Filename(_) => 0xC6,
            Opcode::Read8 => 0xC8,
            Opcode::Read16 => 0xC9,
            Opcode::Read32 => 0xCA,
            Opcode::Readf => 0xCB,
            Opcode::Write8 => 0xCC,
            Opcode::Write16 => 0xCD,
            Opcode::Write32 => 0xCE,
            Opcode::Writef => 0xCF,
            Opcode::ComReady => 0xD0,
            Opcode::ComGet(_) => 0xD3,
            Opcode::ComSet(_) => 0xD4,
            Opcode::ComTest => 0xD5,
            Opcode::MailboxOpen => 0xD8,
            Opcode::MailboxWrite => 0xD9,
            Opcode::MailboxRead => 0xDA,
            Opcode::MailboxTest => 0xDB,
            Opcode::MailboxReady => 0xDC,
            Opcode::MailboxClose => 0xDD,
        }
    }

    pub fn get_subcode_as_u8(&self) -> u8 {
        match self {
            Opcode::ProgramInfo(subcode) => *subcode as u8,
            Opcode::Info(subcode) => *subcode as u8,
            Opcode::String(subcode) => *subcode as u8,
            Opcode::UiRead(subcode) => *subcode as u8,
            Opcode::UiWrite(subcode) => *subcode as u8,
            Opcode::UiButton(subcode) => *subcode as u8,
            Opcode::UiDraw(subcode) => *subcode as u8,
            Opcode::Math(subcode) => *subcode as u8,
            Opcode::ComRead(subcode) => *subcode as u8,
            Opcode::ComWrite(subcode) => *subcode as u8,
            Opcode::Sound(subcode) => *subcode as u8,
            Opcode::InputDevice(subcode) => *subcode as u8,
            Opcode::File(subcode) => *subcode as u8,
            Opcode::Array(subcode) => *subcode as u8,
            Opcode::Filename(subcode) => *subcode as u8,
            Opcode::ComGet(subcode) => *subcode as u8,
            Opcode::ComSet(subcode) => *subcode as u8,
            _ => panic!("Opcode has no subcode"),
        }
    }

    pub fn get_proto(&self) -> &'static [ir::ParamType] {
        use ir::DataType::*;
        use ir::ParamType::*;
        match self {
            Opcode::Error =>
                &[  ],
            Opcode::Nop =>
                &[  ],
            Opcode::ProgramStop =>
                &[ Input(Int16) ],
            Opcode::ProgramStart =>
                &[ Input(Int16), Input(Int32), Input(Int32), Input(Int8) ],
            Opcode::ObjectStop =>
                &[ Input(Int16) ],
            Opcode::ObjectStart =>
                &[ Input(Int16) ],
            Opcode::ObjectTrig =>
                &[ Input(Int16) ],
            Opcode::ObjectWait =>
                &[ Input(Int16) ],
            Opcode::Return =>
                &[  ],
            Opcode::Call =>
                &[ Input(Int16), Input(Int8) ],
            Opcode::ObjectEnd =>
                &[  ],
            Opcode::Sleep =>
                &[  ],
            Opcode::ProgramInfo(subcode) => match subcode {
                ProgramInfoSubcode::OBJ_STOP =>
                    &[ Input(Int16), Input(Int16) ],
                ProgramInfoSubcode::OBJ_START =>
                    &[ Input(Int16), Input(Int16) ],
                ProgramInfoSubcode::GET_STATUS =>
                    &[ Input(Int16), Output(Int8) ],
                ProgramInfoSubcode::GET_SPEED =>
                    &[ Input(Int16), Output(Int32) ],
                ProgramInfoSubcode::GET_PRGRESULT =>
                    &[ Input(Int16), Output(Int8) ],
            },
            Opcode::Label =>
                &[ Input(Int8) ],
            Opcode::Probe =>
                &[ Input(Int16), Input(Int16), Input(Int32), Input(Int32) ],
            Opcode::Do =>
                &[ Input(Int16), Input(Int32), Input(Int32) ],
            Opcode::Add8 =>
                &[ Input(Int8), Input(Int8), Output(Int8) ],
            Opcode::Add16 =>
                &[ Input(Int16), Input(Int16), Output(Int16) ],
            Opcode::Add32 =>
                &[ Input(Int32), Input(Int32), Output(Int32) ],
            Opcode::AddF =>
                &[ Input(Float), Input(Float), Output(Float) ],
            Opcode::Sub8 =>
                &[ Input(Int8), Input(Int8), Output(Int8) ],
            Opcode::Sub16 =>
                &[ Input(Int16), Input(Int16), Output(Int16) ],
            Opcode::Sub32 =>
                &[ Input(Int32), Input(Int32), Output(Int32) ],
            Opcode::SubF =>
                &[ Input(Float), Input(Float), Output(Float) ],
            Opcode::Mul8 =>
                &[ Input(Int8), Input(Int8), Output(Int8) ],
            Opcode::Mul16 =>
                &[ Input(Int16), Input(Int16), Output(Int16) ],
            Opcode::Mul32 =>
                &[ Input(Int32), Input(Int32), Output(Int32) ],
            Opcode::MulF =>
                &[ Input(Float), Input(Float), Output(Float) ],
            Opcode::Div8 =>
                &[ Input(Int8), Input(Int8), Output(Int8) ],
            Opcode::Div16 =>
                &[ Input(Int16), Input(Int16), Output(Int16) ],
            Opcode::Div32 =>
                &[ Input(Int32), Input(Int32), Output(Int32) ],
            Opcode::DivF =>
                &[ Input(Float), Input(Float), Output(Float) ],
            Opcode::Or8 =>
                &[ Input(Int8), Input(Int8), Output(Int8) ],
            Opcode::Or16 =>
                &[ Input(Int16), Input(Int16), Output(Int16) ],
            Opcode::Or32 =>
                &[ Input(Int32), Input(Int32), Output(Int32) ],
            Opcode::And8 =>
                &[ Input(Int8), Input(Int8), Output(Int8) ],
            Opcode::And16 =>
                &[ Input(Int16), Input(Int16), Output(Int16) ],
            Opcode::And32 =>
                &[ Input(Int32), Input(Int32), Output(Int32) ],
            Opcode::Xor8 =>
                &[ Input(Int8), Input(Int8), Output(Int8) ],
            Opcode::Xor16 =>
                &[ Input(Int16), Input(Int16), Output(Int16) ],
            Opcode::Xor32 =>
                &[ Input(Int32), Input(Int32), Output(Int32) ],
            Opcode::Rl8 =>
                &[ Input(Int8), Input(Int8), Output(Int8) ],
            Opcode::Rl16 =>
                &[ Input(Int16), Input(Int16), Output(Int16) ],
            Opcode::Rl32 =>
                &[ Input(Int32), Input(Int32), Output(Int32) ],
            Opcode::InitBytes =>
                &[ Input(Int32), Input(Int8Array), Output(Int8Array) ],
            Opcode::Move8_8 =>
                &[ Input(Int8), Output(Int8) ],
            Opcode::Move8_16 =>
                &[ Input(Int8), Output(Int16) ],
            Opcode::Move8_32 =>
                &[ Input(Int8), Output(Int32) ],
            Opcode::Move8_F =>
                &[ Input(Int8), Output(Float) ],
            Opcode::Move16_8 =>
                &[ Input(Int16), Output(Int8) ],
            Opcode::Move16_16 =>
                &[ Input(Int16), Output(Int16) ],
            Opcode::Move16_32 =>
                &[ Input(Int16), Output(Int32) ],
            Opcode::Move16_F =>
                &[ Input(Int16), Output(Float) ],
            Opcode::Move32_8 =>
                &[ Input(Int32), Output(Int8) ],
            Opcode::Move32_16 =>
                &[ Input(Int32), Output(Int16) ],
            Opcode::Move32_32 =>
                &[ Input(Int32), Output(Int32) ],
            Opcode::Move32_F =>
                &[ Input(Int32), Output(Float) ],
            Opcode::MoveF_8 =>
                &[ Input(Float), Output(Int8) ],
            Opcode::MoveF_16 =>
                &[ Input(Float), Output(Int16) ],
            Opcode::MoveF_32 =>
                &[ Input(Float), Output(Int32) ],
            Opcode::MoveF_F =>
                &[ Input(Float), Output(Float) ],
            Opcode::Jr =>
                &[ Offset ],
            Opcode::JrFalse =>
                &[ Input(Int8), Offset ],
            Opcode::JrTrue =>
                &[ Input(Int8), Offset ],
            Opcode::JrNan =>
                &[ Input(Float), Offset ],
            Opcode::CpLt8 =>
                &[ Input(Int8), Input(Int8), Output(Int8) ],
            Opcode::CpLt16 =>
                &[ Input(Int16), Input(Int16), Output(Int8) ],
            Opcode::CpLt32 =>
                &[ Input(Int32), Input(Int32), Output(Int8) ],
            Opcode::CpLtF =>
                &[ Input(Float), Input(Float), Output(Int8) ],
            Opcode::CpGt8 =>
                &[ Input(Int8), Input(Int8), Output(Int8) ],
            Opcode::CpGt16 =>
                &[ Input(Int16), Input(Int16), Output(Int8) ],
            Opcode::CpGt32 =>
                &[ Input(Int32), Input(Int32), Output(Int8) ],
            Opcode::CpGtF =>
                &[ Input(Float), Input(Float), Output(Int8) ],
            Opcode::CpEq8 =>
                &[ Input(Int8), Input(Int8), Output(Int8) ],
            Opcode::CpEq16 =>
                &[ Input(Int16), Input(Int16), Output(Int8) ],
            Opcode::CpEq32 =>
                &[ Input(Int32), Input(Int32), Output(Int8) ],
            Opcode::CpEqF =>
                &[ Input(Float), Input(Float), Output(Int8) ],
            Opcode::CpNeq8 =>
                &[ Input(Int8), Input(Int8), Output(Int8) ],
            Opcode::CpNeq16 =>
                &[ Input(Int16), Input(Int16), Output(Int8) ],
            Opcode::CpNeq32 =>
                &[ Input(Int32), Input(Int32), Output(Int8) ],
            Opcode::CpNeqF =>
                &[ Input(Float), Input(Float), Output(Int8) ],
            Opcode::CpLteq8 =>
                &[ Input(Int8), Input(Int8), Output(Int8) ],
            Opcode::CpLteq16 =>
                &[ Input(Int16), Input(Int16), Output(Int8) ],
            Opcode::CpLteq32 =>
                &[ Input(Int32), Input(Int32), Output(Int8) ],
            Opcode::CpLteqF =>
                &[ Input(Float), Input(Float), Output(Int8) ],
            Opcode::CpGteq8 =>
                &[ Input(Int8), Input(Int8), Output(Int8) ],
            Opcode::CpGteq16 =>
                &[ Input(Int16), Input(Int16), Output(Int8) ],
            Opcode::CpGteq32 =>
                &[ Input(Int32), Input(Int32), Output(Int8) ],
            Opcode::CpGteqF =>
                &[ Input(Float), Input(Float), Output(Int8) ],
            Opcode::Select8 =>
                &[ Input(Int8), Input(Int8), Input(Int8), Output(Int8) ],
            Opcode::Select16 =>
                &[ Input(Int8), Input(Int16), Input(Int16), Output(Int16) ],
            Opcode::Select32 =>
                &[ Input(Int8), Input(Int32), Input(Int32), Output(Int32) ],
            Opcode::SelectF =>
                &[ Input(Int8), Input(Float), Input(Float), Output(Float) ],
            Opcode::System =>
                &[ Input(Int8), Output(Int32) ],
            Opcode::PortCnvOutput =>
                &[ Input(Int32), Input(Int8), Input(Int8), Input(Int8) ],
            Opcode::PortCnvInput =>
                &[ Input(Int32), Input(Int8), Input(Int8) ],
            Opcode::NoteToFreq =>
                &[ Input(Int8), Output(Int16) ],
            Opcode::JrLt8 =>
                &[ Input(Int8), Input(Int8), Offset ],
            Opcode::JrLt16 =>
                &[ Input(Int16), Input(Int16), Offset ],
            Opcode::JrLt32 =>
                &[ Input(Int32), Input(Int32), Offset ],
            Opcode::JrLtF =>
                &[ Input(Float), Input(Float), Offset ],
            Opcode::JrGt8 =>
                &[ Input(Int8), Input(Int8), Offset ],
            Opcode::JrGt16 =>
                &[ Input(Int16), Input(Int16), Offset ],
            Opcode::JrGt32 =>
                &[ Input(Int32), Input(Int32), Offset ],
            Opcode::JrGtF =>
                &[ Input(Float), Input(Float), Offset ],
            Opcode::JrEq8 =>
                &[ Input(Int8), Input(Int8), Offset ],
            Opcode::JrEq16 =>
                &[ Input(Int16), Input(Int16), Offset ],
            Opcode::JrEq32 =>
                &[ Input(Int32), Input(Int32), Offset ],
            Opcode::JrEqF =>
                &[ Input(Float), Input(Float), Offset ],
            Opcode::JrNeq8 =>
                &[ Input(Int8), Input(Int8), Offset ],
            Opcode::JrNeq16 =>
                &[ Input(Int16), Input(Int16), Offset ],
            Opcode::JrNeq32 =>
                &[ Input(Int32), Input(Int32), Offset ],
            Opcode::JrNeqF =>
                &[ Input(Float), Input(Float), Offset ],
            Opcode::JrLteq8 =>
                &[ Input(Int8), Input(Int8), Offset ],
            Opcode::JrLteq16 =>
                &[ Input(Int16), Input(Int16), Offset ],
            Opcode::JrLteq32 =>
                &[ Input(Int32), Input(Int32), Offset ],
            Opcode::JrLteqF =>
                &[ Input(Float), Input(Float), Offset ],
            Opcode::JrGteq8 =>
                &[ Input(Int8), Input(Int8), Offset ],
            Opcode::JrGteq16 =>
                &[ Input(Int16), Input(Int16), Offset ],
            Opcode::JrGteq32 =>
                &[ Input(Int32), Input(Int32), Offset ],
            Opcode::JrGteqF =>
                &[ Input(Float), Input(Float), Offset ],
            Opcode::Info(subcode) => match subcode {
                InfoSubcode::SET_ERROR =>
                    &[ Input(Int8) ],
                InfoSubcode::GET_ERROR =>
                    &[ Input(Int8) ],
                InfoSubcode::ERRORTEXT =>
                    &[ Input(Int8), Input(Int8), Output(Int8) ],
                InfoSubcode::GET_VOLUME =>
                    &[ Output(Int8) ],
                InfoSubcode::SET_VOLUME =>
                    &[ Input(Int8) ],
                InfoSubcode::GET_MINUTES =>
                    &[ Output(Int8) ],
                InfoSubcode::SET_MINUTES =>
                    &[ Input(Int8) ],
            },
            Opcode::String(subcode) => match subcode {
                StringSubcode::GET_SIZE =>
                    &[ Input(String(0)), Output(Int16) ],
                StringSubcode::ADD =>
                    &[ Input(String(0)), Input(String(0)), Output(String(0)) ],
                StringSubcode::COMPARE =>
                    &[ Input(String(0)), Input(String(0)), Output(Int8) ],
                StringSubcode::DUPLICATE =>
                    &[ Input(String(0)), Output(String(0)) ],
                StringSubcode::VALUE_TO_STRING =>
                    &[ Input(Float), Input(Int8), Input(Int8), Output(String(0)) ],
                StringSubcode::STRING_TO_VALUE =>
                    &[ Input(String(0)), Output(Float) ],
                StringSubcode::STRIP =>
                    &[ Input(String(0)), Output(String(0)) ],
                StringSubcode::NUMBER_TO_STRING =>
                    &[ Input(Int16), Input(Int8), Output(String(0)) ],
                StringSubcode::SUB =>
                    &[ Input(String(0)), Input(String(0)), Output(String(0)) ],
                StringSubcode::VALUE_FORMATTED =>
                    &[ Input(Float), Input(Int8), Input(Int8), Output(String(0)) ],
                StringSubcode::NUMBER_FORMATTED =>
                    &[ Input(Int32), Input(Int8), Input(Int8), Output(String(0)) ],
            },
            Opcode::MemoryWrite =>
                &[ Input(Int16), Input(Int16), Input(Int32), Input(Int32), Input(Int8Array) ],
            Opcode::MemoryRead =>
                &[ Input(Int16), Input(Int16), Input(Int32), Input(Int32), Output(Int8Array) ],
            Opcode::UiFlush =>
                &[  ],
            Opcode::UiRead(subcode) => match subcode {
                UiReadSubcode::GET_VBATT =>
                    &[ Output(Float) ],
                UiReadSubcode::GET_IBATT =>
                    &[ Output(Float) ],
                UiReadSubcode::GET_OS_VERS =>
                    &[ Input(Int8), Output(Int8Array) ],
                UiReadSubcode::GET_EVENT =>
                    &[ Output(Int8) ],
                UiReadSubcode::GET_TBATT =>
                    &[ Output(Float) ],
                UiReadSubcode::GET_IMOTOR =>
                    &[ Output(Float) ],
                UiReadSubcode::GET_STRING =>
                    &[ Input(Int8), Output(Int8Array) ],
                UiReadSubcode::GET_HW_VERS =>
                    &[ Input(Int32), Output(Int8Array) ],
                UiReadSubcode::GET_FW_VERS =>
                    &[ Input(Int32), Output(Int8Array) ],
                UiReadSubcode::GET_FW_BUILD =>
                    &[ Input(Int32), Output(Int8Array) ],
                UiReadSubcode::GET_OS_BUILD =>
                    &[ Input(Int32), Output(Int8Array) ],
                UiReadSubcode::GET_ADDRESS =>
                    &[ Input(Int32) ],
                UiReadSubcode::GET_CODE =>
                    &[ Input(Int32), Output(Int32), Output(Int32), Output(Int8) ],
                UiReadSubcode::KEY =>
                    &[ Input(Int8) ],
                UiReadSubcode::GET_SHUTDOWN =>
                    &[ Output(Int8) ],
                UiReadSubcode::GET_WARNING =>
                    &[ Output(Int8) ],
                UiReadSubcode::GET_LBATT =>
                    &[ Output(Int8) ],
                UiReadSubcode::TEXTBOX_READ =>
                    &[ Input(String(0)), Input(Int32), Input(Int8), Input(Int8), Input(Int16), Output(Int8Array) ],
                UiReadSubcode::GET_VERSION =>
                    &[ Input(Int32), Output(Int8Array) ],
                UiReadSubcode::GET_IP =>
                    &[ Input(Int32), Output(Int8Array) ],
                UiReadSubcode::GET_SDCARD =>
                    &[ Output(Int8), Output(Int32), Output(Int32) ],
                UiReadSubcode::GET_USBSTICK =>
                    &[ Output(Int8), Output(Int32), Output(Int32) ],
            },
            Opcode::UiWrite(subcode) => match subcode {
                UiWriteSubcode::WRITE_FLUSH =>
                    &[  ],
                UiWriteSubcode::FLOATVALUE =>
                    &[ Input(Float), Input(Int8), Input(Int8), Output(String(0)) ],
                UiWriteSubcode::PUT_STRING =>
                    &[ Input(String(0)) ],
                UiWriteSubcode::VALUE8 =>
                    &[ Input(Int8) ],
                UiWriteSubcode::VALUE16 =>
                    &[ Input(Int16) ],
                UiWriteSubcode::VALUE32 =>
                    &[ Input(Int32) ],
                UiWriteSubcode::VALUEF =>
                    &[ Input(Float) ],
                UiWriteSubcode::DOWNLOAD_END =>
                    &[  ],
                UiWriteSubcode::SCREEN_BLOCK =>
                    &[ Input(Int8) ],
                UiWriteSubcode::TEXTBOX_APPEND =>
                    &[ Input(String(0)), Input(Int32), Input(Int8), Input(Int8) ],
                UiWriteSubcode::SET_BUSY =>
                    &[ Input(Int8) ],
                UiWriteSubcode::SET_TESTPIN =>
                    &[ Input(Int8) ],
                UiWriteSubcode::INIT_RUN =>
                    &[  ],
                UiWriteSubcode::LED =>
                    &[ Input(Int8) ],
                UiWriteSubcode::POWER =>
                    &[ Input(Int8) ],
                UiWriteSubcode::GRAPH_SAMPLE =>
                    &[  ],
                UiWriteSubcode::TERMINAL =>
                    &[ Input(Int8) ],
            },
            Opcode::UiButton(subcode) => match subcode {
                UiButtonSubcode::SHORTPRESS =>
                    &[ Input(Int8), Output(Int8) ],
                UiButtonSubcode::LONGPRESS =>
                    &[ Input(Int8), Output(Int8) ],
                UiButtonSubcode::WAIT_FOR_PRESS =>
                    &[  ],
                UiButtonSubcode::FLUSH =>
                    &[  ],
                UiButtonSubcode::PRESS =>
                    &[ Input(Int8) ],
                UiButtonSubcode::RELEASE =>
                    &[ Input(Int8) ],
                UiButtonSubcode::GET_HORZ =>
                    &[ Output(Int16) ],
                UiButtonSubcode::GET_VERT =>
                    &[ Output(Int16) ],
                UiButtonSubcode::PRESSED =>
                    &[ Input(Int8), Output(Int8) ],
                UiButtonSubcode::SET_BACK_BLOCK =>
                    &[ Input(Int8) ],
                UiButtonSubcode::GET_BACK_BLOCK =>
                    &[ Output(Int8) ],
                UiButtonSubcode::TESTSHORTPRESS =>
                    &[ Input(Int8), Output(Int8) ],
                UiButtonSubcode::TESTLONGPRESS =>
                    &[ Input(Int8), Output(Int8) ],
                UiButtonSubcode::GET_BUMBED =>
                    &[ Input(Int8), Output(Int8) ],
                UiButtonSubcode::GET_CLICK =>
                    &[ Output(Int8) ],
            },
            Opcode::UiDraw(subcode) => match subcode {
                UiDrawSubcode::UPDATE =>
                    &[  ],
                UiDrawSubcode::PIXEL =>
                    &[ Input(Int8), Input(Int16), Input(Int16) ],
                UiDrawSubcode::LINE =>
                    &[ Input(Int8), Input(Int16), Input(Int16), Input(Int16), Input(Int16) ],
                UiDrawSubcode::CIRCLE =>
                    &[ Input(Int8), Input(Int16), Input(Int16), Input(Int16) ],
                UiDrawSubcode::TEXT =>
                    &[ Input(Int8), Input(Int16), Input(Int16), Input(String(0)) ],
                UiDrawSubcode::ICON =>
                    &[ Input(Int8), Input(Int16), Input(Int16), Input(Int8), Input(Int32) ],
                UiDrawSubcode::PICTURE =>
                    &[ Input(Int8), Input(Int16), Input(Int16), Input(Int32) ],
                UiDrawSubcode::VALUE =>
                    &[ Input(Int8), Input(Int16), Input(Int16), Input(Float), Input(Int8), Input(Int8) ],
                UiDrawSubcode::FILLRECT =>
                    &[ Input(Int8), Input(Int16), Input(Int16), Input(Int16), Input(Int16) ],
                UiDrawSubcode::RECT =>
                    &[ Input(Int8), Input(Int16), Input(Int16), Input(Int16), Input(Int16) ],
                UiDrawSubcode::NOTIFICATION =>
                    &[ Input(Int8), Input(Int16), Input(Int16), Input(Int8), Input(Int8), Input(Int8), Input(String(0)), Input(Int8) ],
                UiDrawSubcode::QUESTION =>
                    &[ Input(Int8), Input(Int16), Input(Int16), Input(Int8), Input(Int8), Input(String(0)), Input(Int8), Output(Int8) ],
                UiDrawSubcode::KEYBOARD =>
                    &[ Input(Int8), Input(Int16), Input(Int16), Input(Int8), Input(Int8), Input(Int8), Output(Int8Array) ],
                UiDrawSubcode::BROWSE =>
                    &[ Input(Int8), Input(Int16), Input(Int16), Input(Int16), Input(Int16), Input(Int8), Output(Int8), Output(Int8Array) ],
                UiDrawSubcode::VERTBAR =>
                    &[ Input(Int8), Input(Int16), Input(Int16), Input(Int16), Input(Int16), Input(Int16), Input(Int16), Input(Int16) ],
                UiDrawSubcode::INVERSERECT =>
                    &[ Input(Int16), Input(Int16), Input(Int16), Input(Int16) ],
                UiDrawSubcode::SELECT_FONT =>
                    &[ Input(Int8) ],
                UiDrawSubcode::TOPLINE =>
                    &[ Input(Int8) ],
                UiDrawSubcode::FILLWINDOW =>
                    &[ Input(Int8), Input(Int16), Input(Int16) ],
                UiDrawSubcode::DOTLINE =>
                    &[ Input(Int8), Input(Int16), Input(Int16), Input(Int16), Input(Int16), Input(Int16), Input(Int16) ],
                UiDrawSubcode::VIEW_VALUE =>
                    &[ Input(Int8), Input(Int16), Input(Int16), Input(Float), Input(Int8), Input(Int8) ],
                UiDrawSubcode::VIEW_UNIT =>
                    &[ Input(Int8), Input(Int16), Input(Int16), Input(Float), Input(Int8), Input(Int8), Input(Int8), Input(Int8) ],
                UiDrawSubcode::FILLCIRCLE =>
                    &[ Input(Int8), Input(Int16), Input(Int16), Input(Int16) ],
                UiDrawSubcode::STORE =>
                    &[ Input(Int8) ],
                UiDrawSubcode::RESTORE =>
                    &[ Input(Int8) ],
                UiDrawSubcode::ICON_QUESTION =>
                    &[ Input(Int8), Input(Int16), Input(Int16), Input(Int8), Input(Int32) ],
                UiDrawSubcode::BMPFILE =>
                    &[ Input(Int8), Input(Int16), Input(Int16), Input(String(0)) ],
                UiDrawSubcode::GRAPH_SETUP =>
                    &[ Input(Int16), Input(Int16), Input(Int8), Input(Int16), Input(Int16), Input(Float), Input(Float), Input(Float) ],
                UiDrawSubcode::GRAPH_DRAW =>
                    &[ Input(Int8) ],
                UiDrawSubcode::TEXTBOX =>
                    &[ Input(Int16), Input(Int16), Input(Int16), Input(Int16), Input(String(0)), Input(Int32), Input(Int8), Output(Int16) ],
            },
            Opcode::Bp0 =>
                &[  ],
            Opcode::Bp1 =>
                &[  ],
            Opcode::Bp2 =>
                &[  ],
            Opcode::Bp3 =>
                &[  ],
            Opcode::BpSet =>
                &[ Input(Int16), Input(Int8), Input(Int32) ],
            Opcode::Math(subcode) => match subcode {
                MathSubcode::EXP =>
                    &[ Input(Float), Output(Float) ],
                MathSubcode::MOD =>
                    &[ Input(Float), Input(Float), Output(Float) ],
                MathSubcode::FLOOR =>
                    &[ Input(Float), Output(Float) ],
                MathSubcode::CEIL =>
                    &[ Input(Float), Output(Float) ],
                MathSubcode::ROUND =>
                    &[ Input(Float), Output(Float) ],
                MathSubcode::ABS =>
                    &[ Input(Float), Output(Float) ],
                MathSubcode::NEGATE =>
                    &[ Input(Float), Output(Float) ],
                MathSubcode::SQRT =>
                    &[ Input(Float), Output(Float) ],
                MathSubcode::LOG =>
                    &[ Input(Float), Output(Float) ],
                MathSubcode::LN =>
                    &[ Input(Float), Output(Float) ],
                MathSubcode::SIN =>
                    &[ Input(Float), Output(Float) ],
                MathSubcode::COS =>
                    &[ Input(Float), Output(Float) ],
                MathSubcode::TAN =>
                    &[ Input(Float), Output(Float) ],
                MathSubcode::ASIN =>
                    &[ Input(Float), Output(Float) ],
                MathSubcode::ACOS =>
                    &[ Input(Float), Output(Float) ],
                MathSubcode::ATAN =>
                    &[ Input(Float), Output(Float) ],
                MathSubcode::MOD8 =>
                    &[ Input(Int8), Input(Int8), Output(Int8) ],
                MathSubcode::MOD16 =>
                    &[ Input(Int16), Input(Int16), Output(Int16) ],
                MathSubcode::MOD32 =>
                    &[ Input(Int32), Input(Int32), Output(Int32) ],
                MathSubcode::POW =>
                    &[ Input(Float), Input(Float), Output(Float) ],
                MathSubcode::TRUNC =>
                    &[ Input(Float), Input(Int8), Output(Float) ],
            },
            Opcode::Random =>
                &[ Input(Int16), Input(Int16), Output(Int16) ],
            Opcode::KeepAlive =>
                &[ Input(Int8) ],
            Opcode::ComRead(subcode) => match subcode {
                ComReadSubcode::COMMAND =>
                    &[ Input(Int32), Output(Int32), Output(Int32), Output(Int8) ],
            },
            Opcode::ComWrite(subcode) => match subcode {
                ComWriteSubcode::REPLY =>
                    &[ Output(Int32), Output(Int32) ],
            },
            Opcode::Sound(subcode) => match subcode {
                SoundSubcode::BREAK =>
                    &[  ],
                SoundSubcode::TONE =>
                    &[ Input(Int8), Input(Int16), Input(Int16) ],
                SoundSubcode::PLAY =>
                    &[ Input(Int8), Input(String(0)) ],
                SoundSubcode::REPEAT =>
                    &[ Input(Int8), Input(String(0)) ],
            },
            Opcode::SoundTest =>
                &[ Output(Int8) ],
            Opcode::SoundReady =>
                &[  ],
            Opcode::InputDeviceList =>
                &[ Input(Int8), Input(Int8Array), Input(Int8) ],
            Opcode::InputDevice(subcode) => match subcode {
                InputDeviceSubcode::GET_FORMAT =>
                    &[ Input(Int8), Input(Int8), Output(Int8), Output(Int8), Output(Int8), Output(Int8) ],
                InputDeviceSubcode::CAL_MINMAX =>
                    &[ Input(Int8), Input(Int8), Input(Int32), Input(Int32) ],
                InputDeviceSubcode::CAL_DEFAULT =>
                    &[ Input(Int8), Input(Int8) ],
                InputDeviceSubcode::GET_TYPEMODE =>
                    &[ Input(Int8), Input(Int8), Output(Int8), Output(Int8) ],
                InputDeviceSubcode::GET_SYMBOL =>
                    &[ Input(Int8), Input(Int8), Input(Int8), Output(Int8) ],
                InputDeviceSubcode::CAL_MIN =>
                    &[ Input(Int8), Input(Int8), Input(Int32) ],
                InputDeviceSubcode::CAL_MAX =>
                    &[ Input(Int8), Input(Int8), Input(Int32) ],
                InputDeviceSubcode::SETUP =>
                    &[ Input(Int8), Input(Int8), Input(Int8), Input(Int16), Input(Int8), Input(Int8), Output(Handle) ],
                InputDeviceSubcode::CLR_ALL =>
                    &[ Input(Int8) ],
                InputDeviceSubcode::GET_RAW =>
                    &[ Input(Int8), Input(Int8), Output(Int32) ],
                InputDeviceSubcode::GET_CONNECTION =>
                    &[ Input(Int8), Input(Int8), Output(Int8) ],
                InputDeviceSubcode::STOP_ALL =>
                    &[ Input(Int8) ],
                InputDeviceSubcode::GET_NAME =>
                    &[ Input(Int8), Input(Int8), Input(Int8), Output(String(0)) ],
                InputDeviceSubcode::GET_MODENAME =>
                    &[ Input(Int8), Input(Int8), Input(Int8), Input(Int8), Output(String(0)) ],
                InputDeviceSubcode::GET_FIGURES =>
                    &[ Input(Int8), Input(Int8), Output(Int8), Output(Int8) ],
                InputDeviceSubcode::GET_CHANGES =>
                    &[ Input(Int8), Input(Int8), Output(Int8) ],
                InputDeviceSubcode::CLR_CHANGES =>
                    &[ Input(Int8), Input(Int8) ],
                InputDeviceSubcode::READY_PCT =>
                    &[ Input(Int8), Input(Int8), Input(Int8), Input(Int8), Input(Int8), Output(Int8Array) ],
                InputDeviceSubcode::READY_RAW =>
                    &[ Input(Int8), Input(Int8), Input(Int8), Input(Int8), Input(Int8), Output(Int32Array) ],
                InputDeviceSubcode::READY_SI =>
                    &[ Input(Int8), Input(Int8), Input(Int8), Input(Int8), Input(Int8), Output(FloatArray) ],
                InputDeviceSubcode::GET_MINMAX =>
                    &[ Input(Int8), Input(Int8), Output(Float), Output(Float) ],
                InputDeviceSubcode::GET_BUMPS =>
                    &[ Input(Int8), Input(Int8), Output(Float) ],
            },
            Opcode::InputRead =>
                &[ Input(Int8), Input(Int8), Input(Int8), Input(Int8), Output(Int8) ],
            Opcode::InputTest =>
                &[ Input(Int8), Input(Int8), Output(Int8) ],
            Opcode::InputReady =>
                &[ Input(Int8), Input(Int8) ],
            Opcode::InputReadSi =>
                &[ Input(Int8), Input(Int8), Input(Int8), Input(Int8), Output(Float) ],
            Opcode::InputReadExt =>
                &[ Input(Int8), Input(Int8), Input(Int8), Input(Int8), Input(Int8), Input(Int8), Output(Int32Array) ],
            Opcode::InputWrite =>
                &[ Input(Int8), Input(Int8), Input(Int8), Input(Int8Array) ],
            Opcode::OutputSetType =>
                &[ Input(Int8), Input(Int8), Input(Int8) ],
            Opcode::OutputReset =>
                &[ Input(Int8), Input(Int8) ],
            Opcode::OutputStop =>
                &[ Input(Int8), Input(Int8), Input(Int8) ],
            Opcode::OutputPower =>
                &[ Input(Int8), Input(Int8), Input(Int8) ],
            Opcode::OutputSpeed =>
                &[ Input(Int8), Input(Int8), Input(Int8) ],
            Opcode::OutputStart =>
                &[ Input(Int8), Input(Int8) ],
            Opcode::OutputPolarity =>
                &[ Input(Int8), Input(Int8), Input(Int8) ],
            Opcode::OutputRead =>
                &[ Input(Int8), Input(Int8), Input(Int8), Input(Int32) ],
            Opcode::OutputTest =>
                &[ Input(Int8), Input(Int8), Output(Int8) ],
            Opcode::OutputReady =>
                &[ Input(Int8), Input(Int8) ],
            Opcode::OutputStepPower =>
                &[ Input(Int8), Input(Int8), Input(Int8), Input(Int32), Input(Int32), Input(Int32), Input(Int8) ],
            Opcode::OutputTimePower =>
                &[ Input(Int8), Input(Int8), Input(Int8), Input(Int32), Input(Int32), Input(Int32), Input(Int8) ],
            Opcode::OutputStepSpeed =>
                &[ Input(Int8), Input(Int8), Input(Int8), Input(Int32), Input(Int32), Input(Int32), Input(Int8) ],
            Opcode::OutputTimeSpeed =>
                &[ Input(Int8), Input(Int8), Input(Int8), Input(Int32), Input(Int32), Input(Int32), Input(Int8) ],
            Opcode::OutputStepSync =>
                &[ Input(Int8), Input(Int8), Input(Int8), Input(Int16), Input(Int32), Input(Int8) ],
            Opcode::OutputTimeSync =>
                &[ Input(Int8), Input(Int8), Input(Int8), Input(Int16), Input(Int32), Input(Int8) ],
            Opcode::OutputClrCount =>
                &[ Input(Int8), Input(Int8) ],
            Opcode::OutputGetCount =>
                &[ Input(Int8), Input(Int8), Input(Int32) ],
            Opcode::OutputPrgStop =>
                &[  ],
            Opcode::File(subcode) => match subcode {
                FileSubcode::OPEN_APPEND =>
                    &[ Input(String(0)), Output(Handle) ],
                FileSubcode::OPEN_READ =>
                    &[ Input(String(0)), Output(Handle), Output(Int32) ],
                FileSubcode::OPEN_WRITE =>
                    &[ Input(String(0)), Output(Handle) ],
                FileSubcode::READ_VALUE =>
                    &[ Input(Handle), Input(Int8), Output(Float) ],
                FileSubcode::WRITE_VALUE =>
                    &[ Input(Handle), Output(Int8), Output(Float), Output(Int8), Output(Int8) ],
                FileSubcode::READ_TEXT =>
                    &[ Input(Handle), Input(Int8), Input(Int8), Output(Int8Array) ],
                FileSubcode::WRITE_TEXT =>
                    &[ Input(Handle), Input(Int8), Input(String(0)) ],
                FileSubcode::CLOSE =>
                    &[ Input(Handle) ],
                FileSubcode::LOAD_IMAGE =>
                    &[ Input(Int16), Input(String(0)), Output(Int32), Output(Int32) ],
                FileSubcode::GET_HANDLE =>
                    &[ Input(String(0)), Output(Handle), Output(Int8) ],
                FileSubcode::MAKE_FOLDER =>
                    &[ Input(String(0)), Output(Int8) ],
                FileSubcode::GET_POOL =>
                    &[ Input(String(0)), Output(Handle), Output(Int32) ],
                FileSubcode::SET_LOG_SYNC_TIME =>
                    &[ Input(Int32), Input(Int32) ],
                FileSubcode::GET_FOLDERS =>
                    &[ Input(String(0)), Output(Int8) ],
                FileSubcode::GET_LOG_SYNC_TIME =>
                    &[ Input(Int32), Input(Int32) ],
                FileSubcode::GET_SUBFOLDER_NAME =>
                    &[ Input(String(0)), Input(Int8), Input(Int8), Output(Int8Array) ],
                FileSubcode::WRITE_LOG =>
                    &[ Input(Handle), Input(Int32), Input(Int8), Input(Float) ],
                FileSubcode::CLOSE_LOG =>
                    &[ Input(Handle), Input(String(0)) ],
                FileSubcode::GET_IMAGE =>
                    &[ Input(String(0)), Input(Int16), Input(Int8), Output(Int32) ],
                FileSubcode::GET_ITEM =>
                    &[ Input(String(0)), Input(String(0)), Output(Int8) ],
                FileSubcode::GET_CACHE_FILES =>
                    &[ Output(Int8) ],
                FileSubcode::PUT_CACHE_FILE =>
                    &[ Input(String(0)) ],
                FileSubcode::GET_CACHE_FILE =>
                    &[ Input(Int8), Input(Int8), Output(Int8Array) ],
                FileSubcode::DEL_CACHE_FILE =>
                    &[ Input(Int8), Input(Int8), Output(Int8Array) ],
                FileSubcode::DEL_SUBFOLDER =>
                    &[ Input(String(0)), Input(Int8) ],
                FileSubcode::GET_LOG_NAME =>
                    &[ Input(Int8), Input(String(0)) ],
                FileSubcode::OPEN_LOG =>
                    &[ Input(String(0)), Input(Int32), Input(Int32), Input(Int32), Input(Int32), Input(Int32), Input(String(0)), Output(Handle) ],
                FileSubcode::READ_BYTES =>
                    &[ Input(Handle), Input(Int16), Output(Int8Array) ],
                FileSubcode::WRITE_BYTES =>
                    &[ Input(Handle), Input(Int16), Input(Int8Array) ],
                FileSubcode::REMOVE =>
                    &[ Input(Handle) ],
                FileSubcode::MOVE =>
                    &[ Input(String(0)), Input(String(0)) ],
            },
            Opcode::Array(subcode) => match subcode {
                ArraySubcode::DELETE =>
                    &[ Input(Handle) ],
                ArraySubcode::CREATE8 =>
                    &[ Input(Int32), Output(Handle) ],
                ArraySubcode::CREATE16 =>
                    &[ Input(Int32), Output(Handle) ],
                ArraySubcode::CREATE32 =>
                    &[ Input(Int32), Output(Handle) ],
                ArraySubcode::CREATEF =>
                    &[ Input(Int32), Output(Handle) ],
                ArraySubcode::RESIZE =>
                    &[ Input(Handle), Input(Int32) ],
                ArraySubcode::FILL =>
                    &[ Input(Handle), Input(Int32) ],
                ArraySubcode::COPY =>
                    &[ Input(Handle), Input(Handle) ],
                ArraySubcode::INIT8 =>
                    &[ Input(Handle), Input(Int32), Input(Int32), Input(Int8Array) ],
                ArraySubcode::INIT16 =>
                    &[ Input(Handle), Input(Int32), Input(Int32), Input(Int16Array) ],
                ArraySubcode::INIT32 =>
                    &[ Input(Handle), Input(Int32), Input(Int32), Input(Int32Array) ],
                ArraySubcode::INITF =>
                    &[ Input(Handle), Input(Int32), Input(Int32), Input(FloatArray) ],
                ArraySubcode::SIZE =>
                    &[ Input(Handle), Output(Int32) ],
                ArraySubcode::READ_CONTENT =>
                    &[ Input(Handle), Input(Int32), Input(Int32), Output(Int8Array) ],
                ArraySubcode::WRITE_CONTENT =>
                    &[ Input(Int16), Input(Handle), Input(Int32), Input(Int32), Input(Int8Array) ],
                ArraySubcode::READ_SIZE =>
                    &[ Input(Int16), Input(Handle), Output(Int32) ],
            },
            Opcode::ArrayWrite =>
                &[ Input(Handle), Input(Int32), Input(Int32) ],
            Opcode::ArrayRead =>
                &[ Input(Handle), Input(Int32), Input(Int32) ],
            Opcode::ArrayAppend =>
                &[ Input(Handle), Input(Int32) ],
            Opcode::MemoryUsage =>
                &[ Output(Int32), Output(Int32) ],
            Opcode::Filename(subcode) => match subcode {
                FilenameSubcode::EXIST =>
                    &[ Input(String(0)), Output(Int8) ],
                FilenameSubcode::TOTALSIZE =>
                    &[ Input(String(0)), Output(Int32), Output(Int32) ],
                FilenameSubcode::SPLIT =>
                    &[ Input(String(0)), Input(Int8), Input(String(0)), Output(Int8Array), Output(Int8Array) ],
                FilenameSubcode::MERGE =>
                    &[ Input(String(0)), Input(String(0)), Input(String(0)), Input(Int8), Output(Int8Array) ],
                FilenameSubcode::CHECK =>
                    &[ Input(String(0)), Output(Int8) ],
                FilenameSubcode::PACK =>
                    &[ Input(String(0)) ],
                FilenameSubcode::UNPACK =>
                    &[ Input(String(0)) ],
                FilenameSubcode::GET_FOLDERNAME =>
                    &[ Input(Int8), Output(Int8Array) ],
            },
            Opcode::Read8 =>
                &[ Input(Int8Array), Input(Int8), Output(Int8) ],
            Opcode::Read16 =>
                &[ Input(Int16Array), Input(Int8), Output(Int16) ],
            Opcode::Read32 =>
                &[ Input(Int32Array), Input(Int8), Output(Int32) ],
            Opcode::Readf =>
                &[ Input(FloatArray), Input(Int8), Output(Float) ],
            Opcode::Write8 =>
                &[ Input(Int8), Input(Int8), Output(Int8Array) ],
            Opcode::Write16 =>
                &[ Input(Int16), Input(Int8), Output(Int16Array) ],
            Opcode::Write32 =>
                &[ Input(Int32), Input(Int8), Output(Int32Array) ],
            Opcode::Writef =>
                &[ Input(Float), Input(Int8), Output(FloatArray) ],
            Opcode::ComReady =>
                &[ Input(Int8), Input(Int8) ],
            Opcode::ComGet(subcode) => match subcode {
                ComGetSubcode::GET_ON_OFF =>
                    &[ Input(Int8), Output(Int8) ],
                ComGetSubcode::GET_VISIBLE =>
                    &[ Input(Int8), Output(Int8) ],
                ComGetSubcode::GET_RESULT =>
                    &[ Input(Int8), Input(Int8), Output(Int8) ],
                ComGetSubcode::GET_PIN =>
                    &[ Input(Int8), Input(String(0)), Input(Int8), Output(Int8) ],
                ComGetSubcode::SEARCH_ITEMS =>
                    &[ Input(Int8), Output(Int8) ],
                ComGetSubcode::SEARCH_ITEM =>
                    &[ Input(Int8), Input(Int8), Input(Int8), Output(Int8Array), Output(Int8), Output(Int8), Output(Int8), Output(Int8) ],
                ComGetSubcode::FAVOUR_ITEMS =>
                    &[ Input(Int8), Output(Int8) ],
                ComGetSubcode::FAVOUR_ITEM =>
                    &[ Input(Int8), Input(Int8), Input(Int8), Output(Int8Array), Output(Int8), Output(Int8), Output(Int8) ],
                ComGetSubcode::GET_ID =>
                    &[ Input(Int8), Input(Int8), Output(Int8Array) ],
                ComGetSubcode::GET_BRICKNAME =>
                    &[ Input(Int8), Output(Int8Array) ],
                ComGetSubcode::GET_NETWORK =>
                    &[ Input(Int8), Input(Int8), Output(Int8Array), Output(Int8Array), Output(Int8Array) ],
                ComGetSubcode::GET_PRESENT =>
                    &[ Input(Int8), Output(Int8) ],
                ComGetSubcode::GET_ENCRYPT =>
                    &[ Input(Int8), Input(Int8), Output(Int8) ],
                ComGetSubcode::GET_INCOMING =>
                    &[ Input(Int8), Input(Int8), Output(Int8Array) ],
            },
            Opcode::ComSet(subcode) => match subcode {
                ComSetSubcode::SET_ON_OFF =>
                    &[ Input(Int8), Input(Int8) ],
                ComSetSubcode::SET_VISIBLE =>
                    &[ Input(Int8), Input(Int8) ],
                ComSetSubcode::SET_SEARCH =>
                    &[ Input(Int8), Input(Int8) ],
                ComSetSubcode::SET_PIN =>
                    &[ Input(Int8), Input(String(0)), Input(String(0)) ],
                ComSetSubcode::SET_PASSKEY =>
                    &[ Input(Int8), Input(Int8) ],
                ComSetSubcode::SET_CONNECTION =>
                    &[ Input(Int8), Input(String(0)), Input(Int8) ],
                ComSetSubcode::SET_BRICKNAME =>
                    &[ Input(String(0)) ],
                ComSetSubcode::SET_MOVEUP =>
                    &[ Input(Int8), Input(Int8) ],
                ComSetSubcode::SET_MOVEDOWN =>
                    &[ Input(Int8), Input(Int8) ],
                ComSetSubcode::SET_ENCRYPT =>
                    &[ Input(Int8), Input(Int8), Input(Int8) ],
                ComSetSubcode::SET_SSID =>
                    &[ Input(Int8), Input(String(0)) ],
            },
            Opcode::ComTest =>
                &[ Input(Int8), Input(Int8), Output(Int8) ],
            Opcode::MailboxOpen =>
                &[ Input(Int8), Input(Int8), Input(Int8), Input(Int8), Input(Int8), Output(Int8) ],
            Opcode::MailboxWrite =>
                &[ Input(String(0)), Input(Int8), Input(Int8), Input(Int8), Input(Int8) ],
            Opcode::MailboxRead =>
                &[ Input(Int8), Input(Int8), Input(Int8) ],
            Opcode::MailboxTest =>
                &[ Input(Int8), Output(Int8) ],
            Opcode::MailboxReady =>
                &[ Input(Int8) ],
            Opcode::MailboxClose =>
                &[ Input(Int8) ],
        }
    }
}

