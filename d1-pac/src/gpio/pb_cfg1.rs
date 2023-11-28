#[doc = "Register `pb_cfg1` reader"]
pub type R = crate::R<PB_CFG1_SPEC>;
#[doc = "Register `pb_cfg1` writer"]
pub type W = crate::W<PB_CFG1_SPEC>;
#[doc = "Field `pb8_select` reader - "]
pub type PB8_SELECT_R = crate::FieldReader<PB8_SELECT_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PB8_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "2: `10`"]
    DMIC_DATA3 = 2,
    #[doc = "3: `11`"]
    PWM5 = 3,
    #[doc = "4: `100`"]
    TWI2_SCK = 4,
    #[doc = "5: `101`"]
    SPI1_HOLD_DBI_DCX_DBI_WRX = 5,
    #[doc = "6: `110`"]
    UART0_TX = 6,
    #[doc = "7: `111`"]
    UART1_TX = 7,
    #[doc = "14: `1110`"]
    PB_EINT8 = 14,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PB8_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PB8_SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PB8_SELECT_A {
    type Ux = u8;
}
impl PB8_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PB8_SELECT_A> {
        match self.bits {
            0 => Some(PB8_SELECT_A::INPUT),
            1 => Some(PB8_SELECT_A::OUTPUT),
            2 => Some(PB8_SELECT_A::DMIC_DATA3),
            3 => Some(PB8_SELECT_A::PWM5),
            4 => Some(PB8_SELECT_A::TWI2_SCK),
            5 => Some(PB8_SELECT_A::SPI1_HOLD_DBI_DCX_DBI_WRX),
            6 => Some(PB8_SELECT_A::UART0_TX),
            7 => Some(PB8_SELECT_A::UART1_TX),
            14 => Some(PB8_SELECT_A::PB_EINT8),
            15 => Some(PB8_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PB8_SELECT_A::INPUT
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PB8_SELECT_A::OUTPUT
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_dmic_data3(&self) -> bool {
        *self == PB8_SELECT_A::DMIC_DATA3
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_pwm5(&self) -> bool {
        *self == PB8_SELECT_A::PWM5
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_twi2_sck(&self) -> bool {
        *self == PB8_SELECT_A::TWI2_SCK
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_spi1_hold_dbi_dcx_dbi_wrx(&self) -> bool {
        *self == PB8_SELECT_A::SPI1_HOLD_DBI_DCX_DBI_WRX
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_uart0_tx(&self) -> bool {
        *self == PB8_SELECT_A::UART0_TX
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_uart1_tx(&self) -> bool {
        *self == PB8_SELECT_A::UART1_TX
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_pb_eint8(&self) -> bool {
        *self == PB8_SELECT_A::PB_EINT8
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PB8_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pb8_select` writer - "]
pub type PB8_SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PB8_SELECT_A>;
impl<'a, REG> PB8_SELECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(PB8_SELECT_A::INPUT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(PB8_SELECT_A::OUTPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn dmic_data3(self) -> &'a mut crate::W<REG> {
        self.variant(PB8_SELECT_A::DMIC_DATA3)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn pwm5(self) -> &'a mut crate::W<REG> {
        self.variant(PB8_SELECT_A::PWM5)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn twi2_sck(self) -> &'a mut crate::W<REG> {
        self.variant(PB8_SELECT_A::TWI2_SCK)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn spi1_hold_dbi_dcx_dbi_wrx(self) -> &'a mut crate::W<REG> {
        self.variant(PB8_SELECT_A::SPI1_HOLD_DBI_DCX_DBI_WRX)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn uart0_tx(self) -> &'a mut crate::W<REG> {
        self.variant(PB8_SELECT_A::UART0_TX)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart1_tx(self) -> &'a mut crate::W<REG> {
        self.variant(PB8_SELECT_A::UART1_TX)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pb_eint8(self) -> &'a mut crate::W<REG> {
        self.variant(PB8_SELECT_A::PB_EINT8)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PB8_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pb9_select` reader - "]
pub type PB9_SELECT_R = crate::FieldReader<PB9_SELECT_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PB9_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "2: `10`"]
    DMIC_DATA2 = 2,
    #[doc = "3: `11`"]
    PWM6 = 3,
    #[doc = "4: `100`"]
    TWI2_SDA = 4,
    #[doc = "5: `101`"]
    SPI1_MISO_DBI_SDI_DBI_TE_DBI_DCX = 5,
    #[doc = "6: `110`"]
    UART0_RX = 6,
    #[doc = "7: `111`"]
    UART1_RX = 7,
    #[doc = "14: `1110`"]
    PB_EINT9 = 14,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PB9_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PB9_SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PB9_SELECT_A {
    type Ux = u8;
}
impl PB9_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PB9_SELECT_A> {
        match self.bits {
            0 => Some(PB9_SELECT_A::INPUT),
            1 => Some(PB9_SELECT_A::OUTPUT),
            2 => Some(PB9_SELECT_A::DMIC_DATA2),
            3 => Some(PB9_SELECT_A::PWM6),
            4 => Some(PB9_SELECT_A::TWI2_SDA),
            5 => Some(PB9_SELECT_A::SPI1_MISO_DBI_SDI_DBI_TE_DBI_DCX),
            6 => Some(PB9_SELECT_A::UART0_RX),
            7 => Some(PB9_SELECT_A::UART1_RX),
            14 => Some(PB9_SELECT_A::PB_EINT9),
            15 => Some(PB9_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PB9_SELECT_A::INPUT
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PB9_SELECT_A::OUTPUT
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_dmic_data2(&self) -> bool {
        *self == PB9_SELECT_A::DMIC_DATA2
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_pwm6(&self) -> bool {
        *self == PB9_SELECT_A::PWM6
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_twi2_sda(&self) -> bool {
        *self == PB9_SELECT_A::TWI2_SDA
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_spi1_miso_dbi_sdi_dbi_te_dbi_dcx(&self) -> bool {
        *self == PB9_SELECT_A::SPI1_MISO_DBI_SDI_DBI_TE_DBI_DCX
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_uart0_rx(&self) -> bool {
        *self == PB9_SELECT_A::UART0_RX
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_uart1_rx(&self) -> bool {
        *self == PB9_SELECT_A::UART1_RX
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_pb_eint9(&self) -> bool {
        *self == PB9_SELECT_A::PB_EINT9
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PB9_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pb9_select` writer - "]
pub type PB9_SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PB9_SELECT_A>;
impl<'a, REG> PB9_SELECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(PB9_SELECT_A::INPUT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(PB9_SELECT_A::OUTPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn dmic_data2(self) -> &'a mut crate::W<REG> {
        self.variant(PB9_SELECT_A::DMIC_DATA2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn pwm6(self) -> &'a mut crate::W<REG> {
        self.variant(PB9_SELECT_A::PWM6)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn twi2_sda(self) -> &'a mut crate::W<REG> {
        self.variant(PB9_SELECT_A::TWI2_SDA)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn spi1_miso_dbi_sdi_dbi_te_dbi_dcx(self) -> &'a mut crate::W<REG> {
        self.variant(PB9_SELECT_A::SPI1_MISO_DBI_SDI_DBI_TE_DBI_DCX)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn uart0_rx(self) -> &'a mut crate::W<REG> {
        self.variant(PB9_SELECT_A::UART0_RX)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart1_rx(self) -> &'a mut crate::W<REG> {
        self.variant(PB9_SELECT_A::UART1_RX)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pb_eint9(self) -> &'a mut crate::W<REG> {
        self.variant(PB9_SELECT_A::PB_EINT9)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PB9_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pb10_select` reader - "]
pub type PB10_SELECT_R = crate::FieldReader<PB10_SELECT_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PB10_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "2: `10`"]
    DMIC_DATA1 = 2,
    #[doc = "3: `11`"]
    PWM7 = 3,
    #[doc = "4: `100`"]
    TWI0_SCK = 4,
    #[doc = "5: `101`"]
    SPI1_MOSI_DBI_SDO = 5,
    #[doc = "6: `110`"]
    CLK_FANOUT0 = 6,
    #[doc = "7: `111`"]
    UART1_RTS = 7,
    #[doc = "14: `1110`"]
    PB_EINT10 = 14,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PB10_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PB10_SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PB10_SELECT_A {
    type Ux = u8;
}
impl PB10_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PB10_SELECT_A> {
        match self.bits {
            0 => Some(PB10_SELECT_A::INPUT),
            1 => Some(PB10_SELECT_A::OUTPUT),
            2 => Some(PB10_SELECT_A::DMIC_DATA1),
            3 => Some(PB10_SELECT_A::PWM7),
            4 => Some(PB10_SELECT_A::TWI0_SCK),
            5 => Some(PB10_SELECT_A::SPI1_MOSI_DBI_SDO),
            6 => Some(PB10_SELECT_A::CLK_FANOUT0),
            7 => Some(PB10_SELECT_A::UART1_RTS),
            14 => Some(PB10_SELECT_A::PB_EINT10),
            15 => Some(PB10_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PB10_SELECT_A::INPUT
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PB10_SELECT_A::OUTPUT
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_dmic_data1(&self) -> bool {
        *self == PB10_SELECT_A::DMIC_DATA1
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_pwm7(&self) -> bool {
        *self == PB10_SELECT_A::PWM7
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_twi0_sck(&self) -> bool {
        *self == PB10_SELECT_A::TWI0_SCK
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_spi1_mosi_dbi_sdo(&self) -> bool {
        *self == PB10_SELECT_A::SPI1_MOSI_DBI_SDO
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_clk_fanout0(&self) -> bool {
        *self == PB10_SELECT_A::CLK_FANOUT0
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_uart1_rts(&self) -> bool {
        *self == PB10_SELECT_A::UART1_RTS
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_pb_eint10(&self) -> bool {
        *self == PB10_SELECT_A::PB_EINT10
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PB10_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pb10_select` writer - "]
pub type PB10_SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PB10_SELECT_A>;
impl<'a, REG> PB10_SELECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(PB10_SELECT_A::INPUT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(PB10_SELECT_A::OUTPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn dmic_data1(self) -> &'a mut crate::W<REG> {
        self.variant(PB10_SELECT_A::DMIC_DATA1)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn pwm7(self) -> &'a mut crate::W<REG> {
        self.variant(PB10_SELECT_A::PWM7)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn twi0_sck(self) -> &'a mut crate::W<REG> {
        self.variant(PB10_SELECT_A::TWI0_SCK)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn spi1_mosi_dbi_sdo(self) -> &'a mut crate::W<REG> {
        self.variant(PB10_SELECT_A::SPI1_MOSI_DBI_SDO)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn clk_fanout0(self) -> &'a mut crate::W<REG> {
        self.variant(PB10_SELECT_A::CLK_FANOUT0)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart1_rts(self) -> &'a mut crate::W<REG> {
        self.variant(PB10_SELECT_A::UART1_RTS)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pb_eint10(self) -> &'a mut crate::W<REG> {
        self.variant(PB10_SELECT_A::PB_EINT10)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PB10_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pb11_select` reader - "]
pub type PB11_SELECT_R = crate::FieldReader<PB11_SELECT_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PB11_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "2: `10`"]
    DMIC_DATA0 = 2,
    #[doc = "3: `11`"]
    PWM2 = 3,
    #[doc = "4: `100`"]
    TWI0_SDA = 4,
    #[doc = "5: `101`"]
    SPI1_CLK_DBI_SCLK = 5,
    #[doc = "6: `110`"]
    CLK_FANOUT1 = 6,
    #[doc = "7: `111`"]
    UART1_CTS = 7,
    #[doc = "14: `1110`"]
    PB_EINT11 = 14,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PB11_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PB11_SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PB11_SELECT_A {
    type Ux = u8;
}
impl PB11_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PB11_SELECT_A> {
        match self.bits {
            0 => Some(PB11_SELECT_A::INPUT),
            1 => Some(PB11_SELECT_A::OUTPUT),
            2 => Some(PB11_SELECT_A::DMIC_DATA0),
            3 => Some(PB11_SELECT_A::PWM2),
            4 => Some(PB11_SELECT_A::TWI0_SDA),
            5 => Some(PB11_SELECT_A::SPI1_CLK_DBI_SCLK),
            6 => Some(PB11_SELECT_A::CLK_FANOUT1),
            7 => Some(PB11_SELECT_A::UART1_CTS),
            14 => Some(PB11_SELECT_A::PB_EINT11),
            15 => Some(PB11_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PB11_SELECT_A::INPUT
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PB11_SELECT_A::OUTPUT
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_dmic_data0(&self) -> bool {
        *self == PB11_SELECT_A::DMIC_DATA0
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_pwm2(&self) -> bool {
        *self == PB11_SELECT_A::PWM2
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_twi0_sda(&self) -> bool {
        *self == PB11_SELECT_A::TWI0_SDA
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_spi1_clk_dbi_sclk(&self) -> bool {
        *self == PB11_SELECT_A::SPI1_CLK_DBI_SCLK
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_clk_fanout1(&self) -> bool {
        *self == PB11_SELECT_A::CLK_FANOUT1
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_uart1_cts(&self) -> bool {
        *self == PB11_SELECT_A::UART1_CTS
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_pb_eint11(&self) -> bool {
        *self == PB11_SELECT_A::PB_EINT11
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PB11_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pb11_select` writer - "]
pub type PB11_SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PB11_SELECT_A>;
impl<'a, REG> PB11_SELECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(PB11_SELECT_A::INPUT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(PB11_SELECT_A::OUTPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn dmic_data0(self) -> &'a mut crate::W<REG> {
        self.variant(PB11_SELECT_A::DMIC_DATA0)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn pwm2(self) -> &'a mut crate::W<REG> {
        self.variant(PB11_SELECT_A::PWM2)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn twi0_sda(self) -> &'a mut crate::W<REG> {
        self.variant(PB11_SELECT_A::TWI0_SDA)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn spi1_clk_dbi_sclk(self) -> &'a mut crate::W<REG> {
        self.variant(PB11_SELECT_A::SPI1_CLK_DBI_SCLK)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn clk_fanout1(self) -> &'a mut crate::W<REG> {
        self.variant(PB11_SELECT_A::CLK_FANOUT1)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart1_cts(self) -> &'a mut crate::W<REG> {
        self.variant(PB11_SELECT_A::UART1_CTS)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pb_eint11(self) -> &'a mut crate::W<REG> {
        self.variant(PB11_SELECT_A::PB_EINT11)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PB11_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pb12_select` reader - "]
pub type PB12_SELECT_R = crate::FieldReader<PB12_SELECT_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PB12_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "2: `10`"]
    DMIC_CLK = 2,
    #[doc = "3: `11`"]
    PWM0 = 3,
    #[doc = "4: `100`"]
    OWA_IN = 4,
    #[doc = "5: `101`"]
    SPI1_CS_DBI_CSX = 5,
    #[doc = "6: `110`"]
    CLK_FANOUT2 = 6,
    #[doc = "7: `111`"]
    IR_RX = 7,
    #[doc = "14: `1110`"]
    PB_EINT12 = 14,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PB12_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PB12_SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PB12_SELECT_A {
    type Ux = u8;
}
impl PB12_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PB12_SELECT_A> {
        match self.bits {
            0 => Some(PB12_SELECT_A::INPUT),
            1 => Some(PB12_SELECT_A::OUTPUT),
            2 => Some(PB12_SELECT_A::DMIC_CLK),
            3 => Some(PB12_SELECT_A::PWM0),
            4 => Some(PB12_SELECT_A::OWA_IN),
            5 => Some(PB12_SELECT_A::SPI1_CS_DBI_CSX),
            6 => Some(PB12_SELECT_A::CLK_FANOUT2),
            7 => Some(PB12_SELECT_A::IR_RX),
            14 => Some(PB12_SELECT_A::PB_EINT12),
            15 => Some(PB12_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PB12_SELECT_A::INPUT
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PB12_SELECT_A::OUTPUT
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_dmic_clk(&self) -> bool {
        *self == PB12_SELECT_A::DMIC_CLK
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_pwm0(&self) -> bool {
        *self == PB12_SELECT_A::PWM0
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_owa_in(&self) -> bool {
        *self == PB12_SELECT_A::OWA_IN
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_spi1_cs_dbi_csx(&self) -> bool {
        *self == PB12_SELECT_A::SPI1_CS_DBI_CSX
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_clk_fanout2(&self) -> bool {
        *self == PB12_SELECT_A::CLK_FANOUT2
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_ir_rx(&self) -> bool {
        *self == PB12_SELECT_A::IR_RX
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_pb_eint12(&self) -> bool {
        *self == PB12_SELECT_A::PB_EINT12
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PB12_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pb12_select` writer - "]
pub type PB12_SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PB12_SELECT_A>;
impl<'a, REG> PB12_SELECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(PB12_SELECT_A::INPUT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(PB12_SELECT_A::OUTPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn dmic_clk(self) -> &'a mut crate::W<REG> {
        self.variant(PB12_SELECT_A::DMIC_CLK)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn pwm0(self) -> &'a mut crate::W<REG> {
        self.variant(PB12_SELECT_A::PWM0)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn owa_in(self) -> &'a mut crate::W<REG> {
        self.variant(PB12_SELECT_A::OWA_IN)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn spi1_cs_dbi_csx(self) -> &'a mut crate::W<REG> {
        self.variant(PB12_SELECT_A::SPI1_CS_DBI_CSX)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn clk_fanout2(self) -> &'a mut crate::W<REG> {
        self.variant(PB12_SELECT_A::CLK_FANOUT2)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn ir_rx(self) -> &'a mut crate::W<REG> {
        self.variant(PB12_SELECT_A::IR_RX)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pb_eint12(self) -> &'a mut crate::W<REG> {
        self.variant(PB12_SELECT_A::PB_EINT12)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PB12_SELECT_A::IO_DISABLE)
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn pb8_select(&self) -> PB8_SELECT_R {
        PB8_SELECT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn pb9_select(&self) -> PB9_SELECT_R {
        PB9_SELECT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn pb10_select(&self) -> PB10_SELECT_R {
        PB10_SELECT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn pb11_select(&self) -> PB11_SELECT_R {
        PB11_SELECT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn pb12_select(&self) -> PB12_SELECT_R {
        PB12_SELECT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn pb8_select(&mut self) -> PB8_SELECT_W<PB_CFG1_SPEC> {
        PB8_SELECT_W::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn pb9_select(&mut self) -> PB9_SELECT_W<PB_CFG1_SPEC> {
        PB9_SELECT_W::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn pb10_select(&mut self) -> PB10_SELECT_W<PB_CFG1_SPEC> {
        PB10_SELECT_W::new(self, 8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    #[must_use]
    pub fn pb11_select(&mut self) -> PB11_SELECT_W<PB_CFG1_SPEC> {
        PB11_SELECT_W::new(self, 12)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn pb12_select(&mut self) -> PB12_SELECT_W<PB_CFG1_SPEC> {
        PB12_SELECT_W::new(self, 16)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PB Configure Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb_cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb_cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PB_CFG1_SPEC;
impl crate::RegisterSpec for PB_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pb_cfg1::R`](R) reader structure"]
impl crate::Readable for PB_CFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pb_cfg1::W`](W) writer structure"]
impl crate::Writable for PB_CFG1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pb_cfg1 to value 0"]
impl crate::Resettable for PB_CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
