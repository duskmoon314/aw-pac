#[doc = "Register `pc_cfg0` reader"]
pub struct R(crate::R<PC_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PC_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PC_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PC_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pc_cfg0` writer"]
pub struct W(crate::W<PC_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PC_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PC_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PC_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PC7 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PC7_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    SPI0_HOLD = 2,
    #[doc = "4: `100`"]
    UART3_RX = 4,
    #[doc = "6: `110`"]
    TCON_TRIG = 6,
    #[doc = "14: `1110`"]
    PC_EINT7 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    SDC2_D3 = 3,
    #[doc = "5: `101`"]
    TWI3_SDA = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PC7_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PC7_SELECT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PC7_SELECT` reader - PC7 Select"]
pub struct PC7_SELECT_R(crate::FieldReader<u8, PC7_SELECT_A>);
impl PC7_SELECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PC7_SELECT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PC7_SELECT_A> {
        match self.bits {
            0 => Some(PC7_SELECT_A::INPUT),
            2 => Some(PC7_SELECT_A::SPI0_HOLD),
            4 => Some(PC7_SELECT_A::UART3_RX),
            6 => Some(PC7_SELECT_A::TCON_TRIG),
            14 => Some(PC7_SELECT_A::PC_EINT7),
            1 => Some(PC7_SELECT_A::OUTPUT),
            3 => Some(PC7_SELECT_A::SDC2_D3),
            5 => Some(PC7_SELECT_A::TWI3_SDA),
            15 => Some(PC7_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        **self == PC7_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `SPI0_HOLD`"]
    #[inline(always)]
    pub fn is_spi0_hold(&self) -> bool {
        **self == PC7_SELECT_A::SPI0_HOLD
    }
    #[doc = "Checks if the value of the field is `UART3_RX`"]
    #[inline(always)]
    pub fn is_uart3_rx(&self) -> bool {
        **self == PC7_SELECT_A::UART3_RX
    }
    #[doc = "Checks if the value of the field is `TCON_TRIG`"]
    #[inline(always)]
    pub fn is_tcon_trig(&self) -> bool {
        **self == PC7_SELECT_A::TCON_TRIG
    }
    #[doc = "Checks if the value of the field is `PC_EINT7`"]
    #[inline(always)]
    pub fn is_pc_eint7(&self) -> bool {
        **self == PC7_SELECT_A::PC_EINT7
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        **self == PC7_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `SDC2_D3`"]
    #[inline(always)]
    pub fn is_sdc2_d3(&self) -> bool {
        **self == PC7_SELECT_A::SDC2_D3
    }
    #[doc = "Checks if the value of the field is `TWI3_SDA`"]
    #[inline(always)]
    pub fn is_twi3_sda(&self) -> bool {
        **self == PC7_SELECT_A::TWI3_SDA
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        **self == PC7_SELECT_A::IO_DISABLE
    }
}
impl core::ops::Deref for PC7_SELECT_R {
    type Target = crate::FieldReader<u8, PC7_SELECT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC7_SELECT` writer - PC7 Select"]
pub struct PC7_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> PC7_SELECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PC7_SELECT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PC7_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn spi0_hold(self) -> &'a mut W {
        self.variant(PC7_SELECT_A::SPI0_HOLD)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn uart3_rx(self) -> &'a mut W {
        self.variant(PC7_SELECT_A::UART3_RX)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn tcon_trig(self) -> &'a mut W {
        self.variant(PC7_SELECT_A::TCON_TRIG)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pc_eint7(self) -> &'a mut W {
        self.variant(PC7_SELECT_A::PC_EINT7)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PC7_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn sdc2_d3(self) -> &'a mut W {
        self.variant(PC7_SELECT_A::SDC2_D3)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn twi3_sda(self) -> &'a mut W {
        self.variant(PC7_SELECT_A::TWI3_SDA)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PC7_SELECT_A::IO_DISABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
#[doc = "PC6 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PC6_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    SPI0_WP = 2,
    #[doc = "4: `100`"]
    UART3_TX = 4,
    #[doc = "6: `110`"]
    DBG_CLK = 6,
    #[doc = "14: `1110`"]
    PC_EINT6 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    SDC2_D0 = 3,
    #[doc = "5: `101`"]
    TWI3_SCK = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PC6_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PC6_SELECT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PC6_SELECT` reader - PC6 Select"]
pub struct PC6_SELECT_R(crate::FieldReader<u8, PC6_SELECT_A>);
impl PC6_SELECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PC6_SELECT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PC6_SELECT_A> {
        match self.bits {
            0 => Some(PC6_SELECT_A::INPUT),
            2 => Some(PC6_SELECT_A::SPI0_WP),
            4 => Some(PC6_SELECT_A::UART3_TX),
            6 => Some(PC6_SELECT_A::DBG_CLK),
            14 => Some(PC6_SELECT_A::PC_EINT6),
            1 => Some(PC6_SELECT_A::OUTPUT),
            3 => Some(PC6_SELECT_A::SDC2_D0),
            5 => Some(PC6_SELECT_A::TWI3_SCK),
            15 => Some(PC6_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        **self == PC6_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `SPI0_WP`"]
    #[inline(always)]
    pub fn is_spi0_wp(&self) -> bool {
        **self == PC6_SELECT_A::SPI0_WP
    }
    #[doc = "Checks if the value of the field is `UART3_TX`"]
    #[inline(always)]
    pub fn is_uart3_tx(&self) -> bool {
        **self == PC6_SELECT_A::UART3_TX
    }
    #[doc = "Checks if the value of the field is `DBG_CLK`"]
    #[inline(always)]
    pub fn is_dbg_clk(&self) -> bool {
        **self == PC6_SELECT_A::DBG_CLK
    }
    #[doc = "Checks if the value of the field is `PC_EINT6`"]
    #[inline(always)]
    pub fn is_pc_eint6(&self) -> bool {
        **self == PC6_SELECT_A::PC_EINT6
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        **self == PC6_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `SDC2_D0`"]
    #[inline(always)]
    pub fn is_sdc2_d0(&self) -> bool {
        **self == PC6_SELECT_A::SDC2_D0
    }
    #[doc = "Checks if the value of the field is `TWI3_SCK`"]
    #[inline(always)]
    pub fn is_twi3_sck(&self) -> bool {
        **self == PC6_SELECT_A::TWI3_SCK
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        **self == PC6_SELECT_A::IO_DISABLE
    }
}
impl core::ops::Deref for PC6_SELECT_R {
    type Target = crate::FieldReader<u8, PC6_SELECT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC6_SELECT` writer - PC6 Select"]
pub struct PC6_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> PC6_SELECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PC6_SELECT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PC6_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn spi0_wp(self) -> &'a mut W {
        self.variant(PC6_SELECT_A::SPI0_WP)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn uart3_tx(self) -> &'a mut W {
        self.variant(PC6_SELECT_A::UART3_TX)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn dbg_clk(self) -> &'a mut W {
        self.variant(PC6_SELECT_A::DBG_CLK)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pc_eint6(self) -> &'a mut W {
        self.variant(PC6_SELECT_A::PC_EINT6)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PC6_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn sdc2_d0(self) -> &'a mut W {
        self.variant(PC6_SELECT_A::SDC2_D0)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn twi3_sck(self) -> &'a mut W {
        self.variant(PC6_SELECT_A::TWI3_SCK)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PC6_SELECT_A::IO_DISABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "PC5 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PC5_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    SPI0_MISO = 2,
    #[doc = "4: `100`"]
    BOOT_SEL1 = 4,
    #[doc = "14: `1110`"]
    PC_EINT5 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    SDC2_D1 = 3,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PC5_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PC5_SELECT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PC5_SELECT` reader - PC5 Select"]
pub struct PC5_SELECT_R(crate::FieldReader<u8, PC5_SELECT_A>);
impl PC5_SELECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PC5_SELECT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PC5_SELECT_A> {
        match self.bits {
            0 => Some(PC5_SELECT_A::INPUT),
            2 => Some(PC5_SELECT_A::SPI0_MISO),
            4 => Some(PC5_SELECT_A::BOOT_SEL1),
            14 => Some(PC5_SELECT_A::PC_EINT5),
            1 => Some(PC5_SELECT_A::OUTPUT),
            3 => Some(PC5_SELECT_A::SDC2_D1),
            15 => Some(PC5_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        **self == PC5_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `SPI0_MISO`"]
    #[inline(always)]
    pub fn is_spi0_miso(&self) -> bool {
        **self == PC5_SELECT_A::SPI0_MISO
    }
    #[doc = "Checks if the value of the field is `BOOT_SEL1`"]
    #[inline(always)]
    pub fn is_boot_sel1(&self) -> bool {
        **self == PC5_SELECT_A::BOOT_SEL1
    }
    #[doc = "Checks if the value of the field is `PC_EINT5`"]
    #[inline(always)]
    pub fn is_pc_eint5(&self) -> bool {
        **self == PC5_SELECT_A::PC_EINT5
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        **self == PC5_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `SDC2_D1`"]
    #[inline(always)]
    pub fn is_sdc2_d1(&self) -> bool {
        **self == PC5_SELECT_A::SDC2_D1
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        **self == PC5_SELECT_A::IO_DISABLE
    }
}
impl core::ops::Deref for PC5_SELECT_R {
    type Target = crate::FieldReader<u8, PC5_SELECT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC5_SELECT` writer - PC5 Select"]
pub struct PC5_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> PC5_SELECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PC5_SELECT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PC5_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn spi0_miso(self) -> &'a mut W {
        self.variant(PC5_SELECT_A::SPI0_MISO)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn boot_sel1(self) -> &'a mut W {
        self.variant(PC5_SELECT_A::BOOT_SEL1)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pc_eint5(self) -> &'a mut W {
        self.variant(PC5_SELECT_A::PC_EINT5)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PC5_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn sdc2_d1(self) -> &'a mut W {
        self.variant(PC5_SELECT_A::SDC2_D1)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PC5_SELECT_A::IO_DISABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "PC4 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PC4_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    SPI0_MOSI = 2,
    #[doc = "4: `100`"]
    BOOT_SEL0 = 4,
    #[doc = "14: `1110`"]
    PC_EINT4 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    SDC2_D2 = 3,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PC4_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PC4_SELECT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PC4_SELECT` reader - PC4 Select"]
pub struct PC4_SELECT_R(crate::FieldReader<u8, PC4_SELECT_A>);
impl PC4_SELECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PC4_SELECT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PC4_SELECT_A> {
        match self.bits {
            0 => Some(PC4_SELECT_A::INPUT),
            2 => Some(PC4_SELECT_A::SPI0_MOSI),
            4 => Some(PC4_SELECT_A::BOOT_SEL0),
            14 => Some(PC4_SELECT_A::PC_EINT4),
            1 => Some(PC4_SELECT_A::OUTPUT),
            3 => Some(PC4_SELECT_A::SDC2_D2),
            15 => Some(PC4_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        **self == PC4_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `SPI0_MOSI`"]
    #[inline(always)]
    pub fn is_spi0_mosi(&self) -> bool {
        **self == PC4_SELECT_A::SPI0_MOSI
    }
    #[doc = "Checks if the value of the field is `BOOT_SEL0`"]
    #[inline(always)]
    pub fn is_boot_sel0(&self) -> bool {
        **self == PC4_SELECT_A::BOOT_SEL0
    }
    #[doc = "Checks if the value of the field is `PC_EINT4`"]
    #[inline(always)]
    pub fn is_pc_eint4(&self) -> bool {
        **self == PC4_SELECT_A::PC_EINT4
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        **self == PC4_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `SDC2_D2`"]
    #[inline(always)]
    pub fn is_sdc2_d2(&self) -> bool {
        **self == PC4_SELECT_A::SDC2_D2
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        **self == PC4_SELECT_A::IO_DISABLE
    }
}
impl core::ops::Deref for PC4_SELECT_R {
    type Target = crate::FieldReader<u8, PC4_SELECT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC4_SELECT` writer - PC4 Select"]
pub struct PC4_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> PC4_SELECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PC4_SELECT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PC4_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn spi0_mosi(self) -> &'a mut W {
        self.variant(PC4_SELECT_A::SPI0_MOSI)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn boot_sel0(self) -> &'a mut W {
        self.variant(PC4_SELECT_A::BOOT_SEL0)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pc_eint4(self) -> &'a mut W {
        self.variant(PC4_SELECT_A::PC_EINT4)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PC4_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn sdc2_d2(self) -> &'a mut W {
        self.variant(PC4_SELECT_A::SDC2_D2)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PC4_SELECT_A::IO_DISABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "PC3 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PC3_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    SPI0_CS0 = 2,
    #[doc = "14: `1110`"]
    PC_EINT3 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    SDC2_CMD = 3,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PC3_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PC3_SELECT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PC3_SELECT` reader - PC3 Select"]
pub struct PC3_SELECT_R(crate::FieldReader<u8, PC3_SELECT_A>);
impl PC3_SELECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PC3_SELECT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PC3_SELECT_A> {
        match self.bits {
            0 => Some(PC3_SELECT_A::INPUT),
            2 => Some(PC3_SELECT_A::SPI0_CS0),
            14 => Some(PC3_SELECT_A::PC_EINT3),
            1 => Some(PC3_SELECT_A::OUTPUT),
            3 => Some(PC3_SELECT_A::SDC2_CMD),
            15 => Some(PC3_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        **self == PC3_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `SPI0_CS0`"]
    #[inline(always)]
    pub fn is_spi0_cs0(&self) -> bool {
        **self == PC3_SELECT_A::SPI0_CS0
    }
    #[doc = "Checks if the value of the field is `PC_EINT3`"]
    #[inline(always)]
    pub fn is_pc_eint3(&self) -> bool {
        **self == PC3_SELECT_A::PC_EINT3
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        **self == PC3_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `SDC2_CMD`"]
    #[inline(always)]
    pub fn is_sdc2_cmd(&self) -> bool {
        **self == PC3_SELECT_A::SDC2_CMD
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        **self == PC3_SELECT_A::IO_DISABLE
    }
}
impl core::ops::Deref for PC3_SELECT_R {
    type Target = crate::FieldReader<u8, PC3_SELECT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC3_SELECT` writer - PC3 Select"]
pub struct PC3_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> PC3_SELECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PC3_SELECT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PC3_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn spi0_cs0(self) -> &'a mut W {
        self.variant(PC3_SELECT_A::SPI0_CS0)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pc_eint3(self) -> &'a mut W {
        self.variant(PC3_SELECT_A::PC_EINT3)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PC3_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn sdc2_cmd(self) -> &'a mut W {
        self.variant(PC3_SELECT_A::SDC2_CMD)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PC3_SELECT_A::IO_DISABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "PC2 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PC2_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    SPI0_CLK = 2,
    #[doc = "14: `1110`"]
    PC_EINT2 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    SDC2_CLK = 3,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PC2_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PC2_SELECT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PC2_SELECT` reader - PC2 Select"]
pub struct PC2_SELECT_R(crate::FieldReader<u8, PC2_SELECT_A>);
impl PC2_SELECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PC2_SELECT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PC2_SELECT_A> {
        match self.bits {
            0 => Some(PC2_SELECT_A::INPUT),
            2 => Some(PC2_SELECT_A::SPI0_CLK),
            14 => Some(PC2_SELECT_A::PC_EINT2),
            1 => Some(PC2_SELECT_A::OUTPUT),
            3 => Some(PC2_SELECT_A::SDC2_CLK),
            15 => Some(PC2_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        **self == PC2_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `SPI0_CLK`"]
    #[inline(always)]
    pub fn is_spi0_clk(&self) -> bool {
        **self == PC2_SELECT_A::SPI0_CLK
    }
    #[doc = "Checks if the value of the field is `PC_EINT2`"]
    #[inline(always)]
    pub fn is_pc_eint2(&self) -> bool {
        **self == PC2_SELECT_A::PC_EINT2
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        **self == PC2_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `SDC2_CLK`"]
    #[inline(always)]
    pub fn is_sdc2_clk(&self) -> bool {
        **self == PC2_SELECT_A::SDC2_CLK
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        **self == PC2_SELECT_A::IO_DISABLE
    }
}
impl core::ops::Deref for PC2_SELECT_R {
    type Target = crate::FieldReader<u8, PC2_SELECT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC2_SELECT` writer - PC2 Select"]
pub struct PC2_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> PC2_SELECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PC2_SELECT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PC2_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn spi0_clk(self) -> &'a mut W {
        self.variant(PC2_SELECT_A::SPI0_CLK)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pc_eint2(self) -> &'a mut W {
        self.variant(PC2_SELECT_A::PC_EINT2)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PC2_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn sdc2_clk(self) -> &'a mut W {
        self.variant(PC2_SELECT_A::SDC2_CLK)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PC2_SELECT_A::IO_DISABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "PC1 Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PC1_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    UART2_RX = 2,
    #[doc = "14: `1110`"]
    PC_EINT1 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    TWI2_SDA = 3,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PC1_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PC1_SELECT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PC1_SELECT` reader - PC1 Select."]
pub struct PC1_SELECT_R(crate::FieldReader<u8, PC1_SELECT_A>);
impl PC1_SELECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PC1_SELECT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PC1_SELECT_A> {
        match self.bits {
            0 => Some(PC1_SELECT_A::INPUT),
            2 => Some(PC1_SELECT_A::UART2_RX),
            14 => Some(PC1_SELECT_A::PC_EINT1),
            1 => Some(PC1_SELECT_A::OUTPUT),
            3 => Some(PC1_SELECT_A::TWI2_SDA),
            15 => Some(PC1_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        **self == PC1_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `UART2_RX`"]
    #[inline(always)]
    pub fn is_uart2_rx(&self) -> bool {
        **self == PC1_SELECT_A::UART2_RX
    }
    #[doc = "Checks if the value of the field is `PC_EINT1`"]
    #[inline(always)]
    pub fn is_pc_eint1(&self) -> bool {
        **self == PC1_SELECT_A::PC_EINT1
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        **self == PC1_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `TWI2_SDA`"]
    #[inline(always)]
    pub fn is_twi2_sda(&self) -> bool {
        **self == PC1_SELECT_A::TWI2_SDA
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        **self == PC1_SELECT_A::IO_DISABLE
    }
}
impl core::ops::Deref for PC1_SELECT_R {
    type Target = crate::FieldReader<u8, PC1_SELECT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC1_SELECT` writer - PC1 Select."]
pub struct PC1_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> PC1_SELECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PC1_SELECT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PC1_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn uart2_rx(self) -> &'a mut W {
        self.variant(PC1_SELECT_A::UART2_RX)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pc_eint1(self) -> &'a mut W {
        self.variant(PC1_SELECT_A::PC_EINT1)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PC1_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn twi2_sda(self) -> &'a mut W {
        self.variant(PC1_SELECT_A::TWI2_SDA)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PC1_SELECT_A::IO_DISABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "PC0 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PC0_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    UART2_TX = 2,
    #[doc = "4: `100`"]
    LEDC_DO = 4,
    #[doc = "14: `1110`"]
    PC_EINT0 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    TWI2_SCK = 3,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PC0_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PC0_SELECT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PC0_SELECT` reader - PC0 Select"]
pub struct PC0_SELECT_R(crate::FieldReader<u8, PC0_SELECT_A>);
impl PC0_SELECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PC0_SELECT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PC0_SELECT_A> {
        match self.bits {
            0 => Some(PC0_SELECT_A::INPUT),
            2 => Some(PC0_SELECT_A::UART2_TX),
            4 => Some(PC0_SELECT_A::LEDC_DO),
            14 => Some(PC0_SELECT_A::PC_EINT0),
            1 => Some(PC0_SELECT_A::OUTPUT),
            3 => Some(PC0_SELECT_A::TWI2_SCK),
            15 => Some(PC0_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        **self == PC0_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `UART2_TX`"]
    #[inline(always)]
    pub fn is_uart2_tx(&self) -> bool {
        **self == PC0_SELECT_A::UART2_TX
    }
    #[doc = "Checks if the value of the field is `LEDC_DO`"]
    #[inline(always)]
    pub fn is_ledc_do(&self) -> bool {
        **self == PC0_SELECT_A::LEDC_DO
    }
    #[doc = "Checks if the value of the field is `PC_EINT0`"]
    #[inline(always)]
    pub fn is_pc_eint0(&self) -> bool {
        **self == PC0_SELECT_A::PC_EINT0
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        **self == PC0_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `TWI2_SCK`"]
    #[inline(always)]
    pub fn is_twi2_sck(&self) -> bool {
        **self == PC0_SELECT_A::TWI2_SCK
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        **self == PC0_SELECT_A::IO_DISABLE
    }
}
impl core::ops::Deref for PC0_SELECT_R {
    type Target = crate::FieldReader<u8, PC0_SELECT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC0_SELECT` writer - PC0 Select"]
pub struct PC0_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> PC0_SELECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PC0_SELECT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PC0_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn uart2_tx(self) -> &'a mut W {
        self.variant(PC0_SELECT_A::UART2_TX)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn ledc_do(self) -> &'a mut W {
        self.variant(PC0_SELECT_A::LEDC_DO)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pc_eint0(self) -> &'a mut W {
        self.variant(PC0_SELECT_A::PC_EINT0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PC0_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn twi2_sck(self) -> &'a mut W {
        self.variant(PC0_SELECT_A::TWI2_SCK)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PC0_SELECT_A::IO_DISABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - PC7 Select"]
    #[inline(always)]
    pub fn pc7_select(&self) -> PC7_SELECT_R {
        PC7_SELECT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PC6 Select"]
    #[inline(always)]
    pub fn pc6_select(&self) -> PC6_SELECT_R {
        PC6_SELECT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PC5 Select"]
    #[inline(always)]
    pub fn pc5_select(&self) -> PC5_SELECT_R {
        PC5_SELECT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PC4 Select"]
    #[inline(always)]
    pub fn pc4_select(&self) -> PC4_SELECT_R {
        PC4_SELECT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PC3 Select"]
    #[inline(always)]
    pub fn pc3_select(&self) -> PC3_SELECT_R {
        PC3_SELECT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PC2 Select"]
    #[inline(always)]
    pub fn pc2_select(&self) -> PC2_SELECT_R {
        PC2_SELECT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PC1 Select."]
    #[inline(always)]
    pub fn pc1_select(&self) -> PC1_SELECT_R {
        PC1_SELECT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - PC0 Select"]
    #[inline(always)]
    pub fn pc0_select(&self) -> PC0_SELECT_R {
        PC0_SELECT_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - PC7 Select"]
    #[inline(always)]
    pub fn pc7_select(&mut self) -> PC7_SELECT_W {
        PC7_SELECT_W { w: self }
    }
    #[doc = "Bits 24:27 - PC6 Select"]
    #[inline(always)]
    pub fn pc6_select(&mut self) -> PC6_SELECT_W {
        PC6_SELECT_W { w: self }
    }
    #[doc = "Bits 20:23 - PC5 Select"]
    #[inline(always)]
    pub fn pc5_select(&mut self) -> PC5_SELECT_W {
        PC5_SELECT_W { w: self }
    }
    #[doc = "Bits 16:19 - PC4 Select"]
    #[inline(always)]
    pub fn pc4_select(&mut self) -> PC4_SELECT_W {
        PC4_SELECT_W { w: self }
    }
    #[doc = "Bits 12:15 - PC3 Select"]
    #[inline(always)]
    pub fn pc3_select(&mut self) -> PC3_SELECT_W {
        PC3_SELECT_W { w: self }
    }
    #[doc = "Bits 8:11 - PC2 Select"]
    #[inline(always)]
    pub fn pc2_select(&mut self) -> PC2_SELECT_W {
        PC2_SELECT_W { w: self }
    }
    #[doc = "Bits 4:7 - PC1 Select."]
    #[inline(always)]
    pub fn pc1_select(&mut self) -> PC1_SELECT_W {
        PC1_SELECT_W { w: self }
    }
    #[doc = "Bits 0:3 - PC0 Select"]
    #[inline(always)]
    pub fn pc0_select(&mut self) -> PC0_SELECT_W {
        PC0_SELECT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PC Configure Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pc_cfg0](index.html) module"]
pub struct PC_CFG0_SPEC;
impl crate::RegisterSpec for PC_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pc_cfg0::R](R) reader structure"]
impl crate::Readable for PC_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pc_cfg0::W](W) writer structure"]
impl crate::Writable for PC_CFG0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pc_cfg0 to value 0"]
impl crate::Resettable for PC_CFG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
