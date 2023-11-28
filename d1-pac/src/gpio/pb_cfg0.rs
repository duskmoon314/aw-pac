#[doc = "Register `pb_cfg0` reader"]
pub type R = crate::R<PB_CFG0_SPEC>;
#[doc = "Register `pb_cfg0` writer"]
pub type W = crate::W<PB_CFG0_SPEC>;
#[doc = "Field `pb0_select` reader - "]
pub type PB0_SELECT_R = crate::FieldReader<PB0_SELECT_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PB0_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "2: `10`"]
    PWM3 = 2,
    #[doc = "3: `11`"]
    IR_TX = 3,
    #[doc = "4: `100`"]
    TWI2_SCK = 4,
    #[doc = "5: `101`"]
    SPI1_WP_DBI_TE = 5,
    #[doc = "6: `110`"]
    UART0_TX = 6,
    #[doc = "7: `111`"]
    UART2_TX = 7,
    #[doc = "8: `1000`"]
    OWA_OUT = 8,
    #[doc = "14: `1110`"]
    PB_EINT0 = 14,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PB0_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PB0_SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PB0_SELECT_A {
    type Ux = u8;
}
impl PB0_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PB0_SELECT_A> {
        match self.bits {
            0 => Some(PB0_SELECT_A::INPUT),
            1 => Some(PB0_SELECT_A::OUTPUT),
            2 => Some(PB0_SELECT_A::PWM3),
            3 => Some(PB0_SELECT_A::IR_TX),
            4 => Some(PB0_SELECT_A::TWI2_SCK),
            5 => Some(PB0_SELECT_A::SPI1_WP_DBI_TE),
            6 => Some(PB0_SELECT_A::UART0_TX),
            7 => Some(PB0_SELECT_A::UART2_TX),
            8 => Some(PB0_SELECT_A::OWA_OUT),
            14 => Some(PB0_SELECT_A::PB_EINT0),
            15 => Some(PB0_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PB0_SELECT_A::INPUT
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PB0_SELECT_A::OUTPUT
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_pwm3(&self) -> bool {
        *self == PB0_SELECT_A::PWM3
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_ir_tx(&self) -> bool {
        *self == PB0_SELECT_A::IR_TX
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_twi2_sck(&self) -> bool {
        *self == PB0_SELECT_A::TWI2_SCK
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_spi1_wp_dbi_te(&self) -> bool {
        *self == PB0_SELECT_A::SPI1_WP_DBI_TE
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_uart0_tx(&self) -> bool {
        *self == PB0_SELECT_A::UART0_TX
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_uart2_tx(&self) -> bool {
        *self == PB0_SELECT_A::UART2_TX
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_owa_out(&self) -> bool {
        *self == PB0_SELECT_A::OWA_OUT
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_pb_eint0(&self) -> bool {
        *self == PB0_SELECT_A::PB_EINT0
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PB0_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pb0_select` writer - "]
pub type PB0_SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PB0_SELECT_A>;
impl<'a, REG> PB0_SELECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(PB0_SELECT_A::INPUT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(PB0_SELECT_A::OUTPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pwm3(self) -> &'a mut crate::W<REG> {
        self.variant(PB0_SELECT_A::PWM3)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn ir_tx(self) -> &'a mut crate::W<REG> {
        self.variant(PB0_SELECT_A::IR_TX)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn twi2_sck(self) -> &'a mut crate::W<REG> {
        self.variant(PB0_SELECT_A::TWI2_SCK)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn spi1_wp_dbi_te(self) -> &'a mut crate::W<REG> {
        self.variant(PB0_SELECT_A::SPI1_WP_DBI_TE)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn uart0_tx(self) -> &'a mut crate::W<REG> {
        self.variant(PB0_SELECT_A::UART0_TX)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart2_tx(self) -> &'a mut crate::W<REG> {
        self.variant(PB0_SELECT_A::UART2_TX)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn owa_out(self) -> &'a mut crate::W<REG> {
        self.variant(PB0_SELECT_A::OWA_OUT)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pb_eint0(self) -> &'a mut crate::W<REG> {
        self.variant(PB0_SELECT_A::PB_EINT0)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PB0_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pb1_select` reader - "]
pub type PB1_SELECT_R = crate::FieldReader<PB1_SELECT_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PB1_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "2: `10`"]
    PWM4 = 2,
    #[doc = "3: `11`"]
    I2_S2_DOUT3 = 3,
    #[doc = "4: `100`"]
    TWI2_SDA = 4,
    #[doc = "5: `101`"]
    I2_S2_DIN3 = 5,
    #[doc = "6: `110`"]
    UART0_RX = 6,
    #[doc = "7: `111`"]
    UART2_RX = 7,
    #[doc = "8: `1000`"]
    IR_RX = 8,
    #[doc = "14: `1110`"]
    PB_EINT1 = 14,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PB1_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PB1_SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PB1_SELECT_A {
    type Ux = u8;
}
impl PB1_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PB1_SELECT_A> {
        match self.bits {
            0 => Some(PB1_SELECT_A::INPUT),
            1 => Some(PB1_SELECT_A::OUTPUT),
            2 => Some(PB1_SELECT_A::PWM4),
            3 => Some(PB1_SELECT_A::I2_S2_DOUT3),
            4 => Some(PB1_SELECT_A::TWI2_SDA),
            5 => Some(PB1_SELECT_A::I2_S2_DIN3),
            6 => Some(PB1_SELECT_A::UART0_RX),
            7 => Some(PB1_SELECT_A::UART2_RX),
            8 => Some(PB1_SELECT_A::IR_RX),
            14 => Some(PB1_SELECT_A::PB_EINT1),
            15 => Some(PB1_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PB1_SELECT_A::INPUT
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PB1_SELECT_A::OUTPUT
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_pwm4(&self) -> bool {
        *self == PB1_SELECT_A::PWM4
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_i2_s2_dout3(&self) -> bool {
        *self == PB1_SELECT_A::I2_S2_DOUT3
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_twi2_sda(&self) -> bool {
        *self == PB1_SELECT_A::TWI2_SDA
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_i2_s2_din3(&self) -> bool {
        *self == PB1_SELECT_A::I2_S2_DIN3
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_uart0_rx(&self) -> bool {
        *self == PB1_SELECT_A::UART0_RX
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_uart2_rx(&self) -> bool {
        *self == PB1_SELECT_A::UART2_RX
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_ir_rx(&self) -> bool {
        *self == PB1_SELECT_A::IR_RX
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_pb_eint1(&self) -> bool {
        *self == PB1_SELECT_A::PB_EINT1
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PB1_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pb1_select` writer - "]
pub type PB1_SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PB1_SELECT_A>;
impl<'a, REG> PB1_SELECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(PB1_SELECT_A::INPUT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(PB1_SELECT_A::OUTPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pwm4(self) -> &'a mut crate::W<REG> {
        self.variant(PB1_SELECT_A::PWM4)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn i2_s2_dout3(self) -> &'a mut crate::W<REG> {
        self.variant(PB1_SELECT_A::I2_S2_DOUT3)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn twi2_sda(self) -> &'a mut crate::W<REG> {
        self.variant(PB1_SELECT_A::TWI2_SDA)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn i2_s2_din3(self) -> &'a mut crate::W<REG> {
        self.variant(PB1_SELECT_A::I2_S2_DIN3)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn uart0_rx(self) -> &'a mut crate::W<REG> {
        self.variant(PB1_SELECT_A::UART0_RX)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart2_rx(self) -> &'a mut crate::W<REG> {
        self.variant(PB1_SELECT_A::UART2_RX)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn ir_rx(self) -> &'a mut crate::W<REG> {
        self.variant(PB1_SELECT_A::IR_RX)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pb_eint1(self) -> &'a mut crate::W<REG> {
        self.variant(PB1_SELECT_A::PB_EINT1)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PB1_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pb2_select` reader - "]
pub type PB2_SELECT_R = crate::FieldReader<PB2_SELECT_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PB2_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "2: `10`"]
    LCD0_D0 = 2,
    #[doc = "3: `11`"]
    I2_S2_DOUT2 = 3,
    #[doc = "4: `100`"]
    TWI0_SDA = 4,
    #[doc = "5: `101`"]
    I2_S2_DIN2 = 5,
    #[doc = "6: `110`"]
    LCD0_D18 = 6,
    #[doc = "7: `111`"]
    UART4_TX = 7,
    #[doc = "14: `1110`"]
    PB_EINT2 = 14,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PB2_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PB2_SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PB2_SELECT_A {
    type Ux = u8;
}
impl PB2_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PB2_SELECT_A> {
        match self.bits {
            0 => Some(PB2_SELECT_A::INPUT),
            1 => Some(PB2_SELECT_A::OUTPUT),
            2 => Some(PB2_SELECT_A::LCD0_D0),
            3 => Some(PB2_SELECT_A::I2_S2_DOUT2),
            4 => Some(PB2_SELECT_A::TWI0_SDA),
            5 => Some(PB2_SELECT_A::I2_S2_DIN2),
            6 => Some(PB2_SELECT_A::LCD0_D18),
            7 => Some(PB2_SELECT_A::UART4_TX),
            14 => Some(PB2_SELECT_A::PB_EINT2),
            15 => Some(PB2_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PB2_SELECT_A::INPUT
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PB2_SELECT_A::OUTPUT
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_lcd0_d0(&self) -> bool {
        *self == PB2_SELECT_A::LCD0_D0
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_i2_s2_dout2(&self) -> bool {
        *self == PB2_SELECT_A::I2_S2_DOUT2
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_twi0_sda(&self) -> bool {
        *self == PB2_SELECT_A::TWI0_SDA
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_i2_s2_din2(&self) -> bool {
        *self == PB2_SELECT_A::I2_S2_DIN2
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_lcd0_d18(&self) -> bool {
        *self == PB2_SELECT_A::LCD0_D18
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_uart4_tx(&self) -> bool {
        *self == PB2_SELECT_A::UART4_TX
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_pb_eint2(&self) -> bool {
        *self == PB2_SELECT_A::PB_EINT2
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PB2_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pb2_select` writer - "]
pub type PB2_SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PB2_SELECT_A>;
impl<'a, REG> PB2_SELECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(PB2_SELECT_A::INPUT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(PB2_SELECT_A::OUTPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn lcd0_d0(self) -> &'a mut crate::W<REG> {
        self.variant(PB2_SELECT_A::LCD0_D0)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn i2_s2_dout2(self) -> &'a mut crate::W<REG> {
        self.variant(PB2_SELECT_A::I2_S2_DOUT2)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn twi0_sda(self) -> &'a mut crate::W<REG> {
        self.variant(PB2_SELECT_A::TWI0_SDA)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn i2_s2_din2(self) -> &'a mut crate::W<REG> {
        self.variant(PB2_SELECT_A::I2_S2_DIN2)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn lcd0_d18(self) -> &'a mut crate::W<REG> {
        self.variant(PB2_SELECT_A::LCD0_D18)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart4_tx(self) -> &'a mut crate::W<REG> {
        self.variant(PB2_SELECT_A::UART4_TX)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pb_eint2(self) -> &'a mut crate::W<REG> {
        self.variant(PB2_SELECT_A::PB_EINT2)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PB2_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pb3_select` reader - "]
pub type PB3_SELECT_R = crate::FieldReader<PB3_SELECT_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PB3_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "2: `10`"]
    LCD0_D1 = 2,
    #[doc = "3: `11`"]
    I2_S2_DOUT1 = 3,
    #[doc = "4: `100`"]
    TWI0_SCK = 4,
    #[doc = "5: `101`"]
    I2_S2_DIN0 = 5,
    #[doc = "6: `110`"]
    LCD0_D19 = 6,
    #[doc = "7: `111`"]
    UART4_RX = 7,
    #[doc = "14: `1110`"]
    PB_EINT3 = 14,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PB3_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PB3_SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PB3_SELECT_A {
    type Ux = u8;
}
impl PB3_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PB3_SELECT_A> {
        match self.bits {
            0 => Some(PB3_SELECT_A::INPUT),
            1 => Some(PB3_SELECT_A::OUTPUT),
            2 => Some(PB3_SELECT_A::LCD0_D1),
            3 => Some(PB3_SELECT_A::I2_S2_DOUT1),
            4 => Some(PB3_SELECT_A::TWI0_SCK),
            5 => Some(PB3_SELECT_A::I2_S2_DIN0),
            6 => Some(PB3_SELECT_A::LCD0_D19),
            7 => Some(PB3_SELECT_A::UART4_RX),
            14 => Some(PB3_SELECT_A::PB_EINT3),
            15 => Some(PB3_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PB3_SELECT_A::INPUT
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PB3_SELECT_A::OUTPUT
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_lcd0_d1(&self) -> bool {
        *self == PB3_SELECT_A::LCD0_D1
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_i2_s2_dout1(&self) -> bool {
        *self == PB3_SELECT_A::I2_S2_DOUT1
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_twi0_sck(&self) -> bool {
        *self == PB3_SELECT_A::TWI0_SCK
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_i2_s2_din0(&self) -> bool {
        *self == PB3_SELECT_A::I2_S2_DIN0
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_lcd0_d19(&self) -> bool {
        *self == PB3_SELECT_A::LCD0_D19
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_uart4_rx(&self) -> bool {
        *self == PB3_SELECT_A::UART4_RX
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_pb_eint3(&self) -> bool {
        *self == PB3_SELECT_A::PB_EINT3
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PB3_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pb3_select` writer - "]
pub type PB3_SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PB3_SELECT_A>;
impl<'a, REG> PB3_SELECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(PB3_SELECT_A::INPUT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(PB3_SELECT_A::OUTPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn lcd0_d1(self) -> &'a mut crate::W<REG> {
        self.variant(PB3_SELECT_A::LCD0_D1)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn i2_s2_dout1(self) -> &'a mut crate::W<REG> {
        self.variant(PB3_SELECT_A::I2_S2_DOUT1)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn twi0_sck(self) -> &'a mut crate::W<REG> {
        self.variant(PB3_SELECT_A::TWI0_SCK)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn i2_s2_din0(self) -> &'a mut crate::W<REG> {
        self.variant(PB3_SELECT_A::I2_S2_DIN0)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn lcd0_d19(self) -> &'a mut crate::W<REG> {
        self.variant(PB3_SELECT_A::LCD0_D19)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart4_rx(self) -> &'a mut crate::W<REG> {
        self.variant(PB3_SELECT_A::UART4_RX)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pb_eint3(self) -> &'a mut crate::W<REG> {
        self.variant(PB3_SELECT_A::PB_EINT3)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PB3_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pb4_select` reader - "]
pub type PB4_SELECT_R = crate::FieldReader<PB4_SELECT_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PB4_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "2: `10`"]
    LCD0_D8 = 2,
    #[doc = "3: `11`"]
    I2_S2_DOUT0 = 3,
    #[doc = "4: `100`"]
    TWI1_SCK = 4,
    #[doc = "5: `101`"]
    I2_S2_DIN1 = 5,
    #[doc = "6: `110`"]
    LCD0_D20 = 6,
    #[doc = "7: `111`"]
    UART5_TX = 7,
    #[doc = "14: `1110`"]
    PB_EINT4 = 14,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PB4_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PB4_SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PB4_SELECT_A {
    type Ux = u8;
}
impl PB4_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PB4_SELECT_A> {
        match self.bits {
            0 => Some(PB4_SELECT_A::INPUT),
            1 => Some(PB4_SELECT_A::OUTPUT),
            2 => Some(PB4_SELECT_A::LCD0_D8),
            3 => Some(PB4_SELECT_A::I2_S2_DOUT0),
            4 => Some(PB4_SELECT_A::TWI1_SCK),
            5 => Some(PB4_SELECT_A::I2_S2_DIN1),
            6 => Some(PB4_SELECT_A::LCD0_D20),
            7 => Some(PB4_SELECT_A::UART5_TX),
            14 => Some(PB4_SELECT_A::PB_EINT4),
            15 => Some(PB4_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PB4_SELECT_A::INPUT
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PB4_SELECT_A::OUTPUT
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_lcd0_d8(&self) -> bool {
        *self == PB4_SELECT_A::LCD0_D8
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_i2_s2_dout0(&self) -> bool {
        *self == PB4_SELECT_A::I2_S2_DOUT0
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_twi1_sck(&self) -> bool {
        *self == PB4_SELECT_A::TWI1_SCK
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_i2_s2_din1(&self) -> bool {
        *self == PB4_SELECT_A::I2_S2_DIN1
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_lcd0_d20(&self) -> bool {
        *self == PB4_SELECT_A::LCD0_D20
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_uart5_tx(&self) -> bool {
        *self == PB4_SELECT_A::UART5_TX
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_pb_eint4(&self) -> bool {
        *self == PB4_SELECT_A::PB_EINT4
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PB4_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pb4_select` writer - "]
pub type PB4_SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PB4_SELECT_A>;
impl<'a, REG> PB4_SELECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(PB4_SELECT_A::INPUT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(PB4_SELECT_A::OUTPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn lcd0_d8(self) -> &'a mut crate::W<REG> {
        self.variant(PB4_SELECT_A::LCD0_D8)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn i2_s2_dout0(self) -> &'a mut crate::W<REG> {
        self.variant(PB4_SELECT_A::I2_S2_DOUT0)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn twi1_sck(self) -> &'a mut crate::W<REG> {
        self.variant(PB4_SELECT_A::TWI1_SCK)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn i2_s2_din1(self) -> &'a mut crate::W<REG> {
        self.variant(PB4_SELECT_A::I2_S2_DIN1)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn lcd0_d20(self) -> &'a mut crate::W<REG> {
        self.variant(PB4_SELECT_A::LCD0_D20)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart5_tx(self) -> &'a mut crate::W<REG> {
        self.variant(PB4_SELECT_A::UART5_TX)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pb_eint4(self) -> &'a mut crate::W<REG> {
        self.variant(PB4_SELECT_A::PB_EINT4)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PB4_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pb5_select` reader - "]
pub type PB5_SELECT_R = crate::FieldReader<PB5_SELECT_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PB5_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "2: `10`"]
    LCD0_D9 = 2,
    #[doc = "3: `11`"]
    I2_S2_BCLK = 3,
    #[doc = "4: `100`"]
    TWI1_SDA = 4,
    #[doc = "5: `101`"]
    PWM0 = 5,
    #[doc = "6: `110`"]
    LCD0_D21 = 6,
    #[doc = "7: `111`"]
    UART5_RX = 7,
    #[doc = "14: `1110`"]
    PB_EINT5 = 14,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PB5_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PB5_SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PB5_SELECT_A {
    type Ux = u8;
}
impl PB5_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PB5_SELECT_A> {
        match self.bits {
            0 => Some(PB5_SELECT_A::INPUT),
            1 => Some(PB5_SELECT_A::OUTPUT),
            2 => Some(PB5_SELECT_A::LCD0_D9),
            3 => Some(PB5_SELECT_A::I2_S2_BCLK),
            4 => Some(PB5_SELECT_A::TWI1_SDA),
            5 => Some(PB5_SELECT_A::PWM0),
            6 => Some(PB5_SELECT_A::LCD0_D21),
            7 => Some(PB5_SELECT_A::UART5_RX),
            14 => Some(PB5_SELECT_A::PB_EINT5),
            15 => Some(PB5_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PB5_SELECT_A::INPUT
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PB5_SELECT_A::OUTPUT
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_lcd0_d9(&self) -> bool {
        *self == PB5_SELECT_A::LCD0_D9
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_i2_s2_bclk(&self) -> bool {
        *self == PB5_SELECT_A::I2_S2_BCLK
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_twi1_sda(&self) -> bool {
        *self == PB5_SELECT_A::TWI1_SDA
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_pwm0(&self) -> bool {
        *self == PB5_SELECT_A::PWM0
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_lcd0_d21(&self) -> bool {
        *self == PB5_SELECT_A::LCD0_D21
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_uart5_rx(&self) -> bool {
        *self == PB5_SELECT_A::UART5_RX
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_pb_eint5(&self) -> bool {
        *self == PB5_SELECT_A::PB_EINT5
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PB5_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pb5_select` writer - "]
pub type PB5_SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PB5_SELECT_A>;
impl<'a, REG> PB5_SELECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(PB5_SELECT_A::INPUT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(PB5_SELECT_A::OUTPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn lcd0_d9(self) -> &'a mut crate::W<REG> {
        self.variant(PB5_SELECT_A::LCD0_D9)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn i2_s2_bclk(self) -> &'a mut crate::W<REG> {
        self.variant(PB5_SELECT_A::I2_S2_BCLK)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn twi1_sda(self) -> &'a mut crate::W<REG> {
        self.variant(PB5_SELECT_A::TWI1_SDA)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn pwm0(self) -> &'a mut crate::W<REG> {
        self.variant(PB5_SELECT_A::PWM0)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn lcd0_d21(self) -> &'a mut crate::W<REG> {
        self.variant(PB5_SELECT_A::LCD0_D21)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart5_rx(self) -> &'a mut crate::W<REG> {
        self.variant(PB5_SELECT_A::UART5_RX)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pb_eint5(self) -> &'a mut crate::W<REG> {
        self.variant(PB5_SELECT_A::PB_EINT5)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PB5_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pb6_select` reader - "]
pub type PB6_SELECT_R = crate::FieldReader<PB6_SELECT_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PB6_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "2: `10`"]
    LCD0_D16 = 2,
    #[doc = "3: `11`"]
    I2_S2_LRCK = 3,
    #[doc = "4: `100`"]
    TWI3_SCK = 4,
    #[doc = "5: `101`"]
    PWM1 = 5,
    #[doc = "6: `110`"]
    LCD0_D22 = 6,
    #[doc = "7: `111`"]
    UART3_TX = 7,
    #[doc = "8: `1000`"]
    CPUBIST0 = 8,
    #[doc = "14: `1110`"]
    PB_EINT6 = 14,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PB6_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PB6_SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PB6_SELECT_A {
    type Ux = u8;
}
impl PB6_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PB6_SELECT_A> {
        match self.bits {
            0 => Some(PB6_SELECT_A::INPUT),
            1 => Some(PB6_SELECT_A::OUTPUT),
            2 => Some(PB6_SELECT_A::LCD0_D16),
            3 => Some(PB6_SELECT_A::I2_S2_LRCK),
            4 => Some(PB6_SELECT_A::TWI3_SCK),
            5 => Some(PB6_SELECT_A::PWM1),
            6 => Some(PB6_SELECT_A::LCD0_D22),
            7 => Some(PB6_SELECT_A::UART3_TX),
            8 => Some(PB6_SELECT_A::CPUBIST0),
            14 => Some(PB6_SELECT_A::PB_EINT6),
            15 => Some(PB6_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PB6_SELECT_A::INPUT
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PB6_SELECT_A::OUTPUT
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_lcd0_d16(&self) -> bool {
        *self == PB6_SELECT_A::LCD0_D16
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_i2_s2_lrck(&self) -> bool {
        *self == PB6_SELECT_A::I2_S2_LRCK
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_twi3_sck(&self) -> bool {
        *self == PB6_SELECT_A::TWI3_SCK
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == PB6_SELECT_A::PWM1
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_lcd0_d22(&self) -> bool {
        *self == PB6_SELECT_A::LCD0_D22
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_uart3_tx(&self) -> bool {
        *self == PB6_SELECT_A::UART3_TX
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_cpubist0(&self) -> bool {
        *self == PB6_SELECT_A::CPUBIST0
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_pb_eint6(&self) -> bool {
        *self == PB6_SELECT_A::PB_EINT6
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PB6_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pb6_select` writer - "]
pub type PB6_SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PB6_SELECT_A>;
impl<'a, REG> PB6_SELECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(PB6_SELECT_A::INPUT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(PB6_SELECT_A::OUTPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn lcd0_d16(self) -> &'a mut crate::W<REG> {
        self.variant(PB6_SELECT_A::LCD0_D16)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn i2_s2_lrck(self) -> &'a mut crate::W<REG> {
        self.variant(PB6_SELECT_A::I2_S2_LRCK)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn twi3_sck(self) -> &'a mut crate::W<REG> {
        self.variant(PB6_SELECT_A::TWI3_SCK)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut crate::W<REG> {
        self.variant(PB6_SELECT_A::PWM1)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn lcd0_d22(self) -> &'a mut crate::W<REG> {
        self.variant(PB6_SELECT_A::LCD0_D22)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart3_tx(self) -> &'a mut crate::W<REG> {
        self.variant(PB6_SELECT_A::UART3_TX)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn cpubist0(self) -> &'a mut crate::W<REG> {
        self.variant(PB6_SELECT_A::CPUBIST0)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pb_eint6(self) -> &'a mut crate::W<REG> {
        self.variant(PB6_SELECT_A::PB_EINT6)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PB6_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pb7_select` reader - "]
pub type PB7_SELECT_R = crate::FieldReader<PB7_SELECT_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PB7_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "2: `10`"]
    LCD0_D17 = 2,
    #[doc = "3: `11`"]
    I2_S2_MCLK = 3,
    #[doc = "4: `100`"]
    TWI3_SDA = 4,
    #[doc = "5: `101`"]
    IR_RX = 5,
    #[doc = "6: `110`"]
    LCD0_D23 = 6,
    #[doc = "7: `111`"]
    UART3_RX = 7,
    #[doc = "8: `1000`"]
    CPUBIST1 = 8,
    #[doc = "14: `1110`"]
    PB_EINT7 = 14,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PB7_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PB7_SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PB7_SELECT_A {
    type Ux = u8;
}
impl PB7_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PB7_SELECT_A> {
        match self.bits {
            0 => Some(PB7_SELECT_A::INPUT),
            1 => Some(PB7_SELECT_A::OUTPUT),
            2 => Some(PB7_SELECT_A::LCD0_D17),
            3 => Some(PB7_SELECT_A::I2_S2_MCLK),
            4 => Some(PB7_SELECT_A::TWI3_SDA),
            5 => Some(PB7_SELECT_A::IR_RX),
            6 => Some(PB7_SELECT_A::LCD0_D23),
            7 => Some(PB7_SELECT_A::UART3_RX),
            8 => Some(PB7_SELECT_A::CPUBIST1),
            14 => Some(PB7_SELECT_A::PB_EINT7),
            15 => Some(PB7_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PB7_SELECT_A::INPUT
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PB7_SELECT_A::OUTPUT
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_lcd0_d17(&self) -> bool {
        *self == PB7_SELECT_A::LCD0_D17
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_i2_s2_mclk(&self) -> bool {
        *self == PB7_SELECT_A::I2_S2_MCLK
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_twi3_sda(&self) -> bool {
        *self == PB7_SELECT_A::TWI3_SDA
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_ir_rx(&self) -> bool {
        *self == PB7_SELECT_A::IR_RX
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_lcd0_d23(&self) -> bool {
        *self == PB7_SELECT_A::LCD0_D23
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_uart3_rx(&self) -> bool {
        *self == PB7_SELECT_A::UART3_RX
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_cpubist1(&self) -> bool {
        *self == PB7_SELECT_A::CPUBIST1
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_pb_eint7(&self) -> bool {
        *self == PB7_SELECT_A::PB_EINT7
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PB7_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pb7_select` writer - "]
pub type PB7_SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PB7_SELECT_A>;
impl<'a, REG> PB7_SELECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(PB7_SELECT_A::INPUT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(PB7_SELECT_A::OUTPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn lcd0_d17(self) -> &'a mut crate::W<REG> {
        self.variant(PB7_SELECT_A::LCD0_D17)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn i2_s2_mclk(self) -> &'a mut crate::W<REG> {
        self.variant(PB7_SELECT_A::I2_S2_MCLK)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn twi3_sda(self) -> &'a mut crate::W<REG> {
        self.variant(PB7_SELECT_A::TWI3_SDA)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn ir_rx(self) -> &'a mut crate::W<REG> {
        self.variant(PB7_SELECT_A::IR_RX)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn lcd0_d23(self) -> &'a mut crate::W<REG> {
        self.variant(PB7_SELECT_A::LCD0_D23)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart3_rx(self) -> &'a mut crate::W<REG> {
        self.variant(PB7_SELECT_A::UART3_RX)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn cpubist1(self) -> &'a mut crate::W<REG> {
        self.variant(PB7_SELECT_A::CPUBIST1)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pb_eint7(self) -> &'a mut crate::W<REG> {
        self.variant(PB7_SELECT_A::PB_EINT7)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PB7_SELECT_A::IO_DISABLE)
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn pb0_select(&self) -> PB0_SELECT_R {
        PB0_SELECT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn pb1_select(&self) -> PB1_SELECT_R {
        PB1_SELECT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn pb2_select(&self) -> PB2_SELECT_R {
        PB2_SELECT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn pb3_select(&self) -> PB3_SELECT_R {
        PB3_SELECT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn pb4_select(&self) -> PB4_SELECT_R {
        PB4_SELECT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn pb5_select(&self) -> PB5_SELECT_R {
        PB5_SELECT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn pb6_select(&self) -> PB6_SELECT_R {
        PB6_SELECT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn pb7_select(&self) -> PB7_SELECT_R {
        PB7_SELECT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn pb0_select(&mut self) -> PB0_SELECT_W<PB_CFG0_SPEC> {
        PB0_SELECT_W::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn pb1_select(&mut self) -> PB1_SELECT_W<PB_CFG0_SPEC> {
        PB1_SELECT_W::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn pb2_select(&mut self) -> PB2_SELECT_W<PB_CFG0_SPEC> {
        PB2_SELECT_W::new(self, 8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    #[must_use]
    pub fn pb3_select(&mut self) -> PB3_SELECT_W<PB_CFG0_SPEC> {
        PB3_SELECT_W::new(self, 12)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn pb4_select(&mut self) -> PB4_SELECT_W<PB_CFG0_SPEC> {
        PB4_SELECT_W::new(self, 16)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    #[must_use]
    pub fn pb5_select(&mut self) -> PB5_SELECT_W<PB_CFG0_SPEC> {
        PB5_SELECT_W::new(self, 20)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    #[must_use]
    pub fn pb6_select(&mut self) -> PB6_SELECT_W<PB_CFG0_SPEC> {
        PB6_SELECT_W::new(self, 24)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    #[must_use]
    pub fn pb7_select(&mut self) -> PB7_SELECT_W<PB_CFG0_SPEC> {
        PB7_SELECT_W::new(self, 28)
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
#[doc = "PB Configure Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb_cfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb_cfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PB_CFG0_SPEC;
impl crate::RegisterSpec for PB_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pb_cfg0::R`](R) reader structure"]
impl crate::Readable for PB_CFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pb_cfg0::W`](W) writer structure"]
impl crate::Writable for PB_CFG0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pb_cfg0 to value 0"]
impl crate::Resettable for PB_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
