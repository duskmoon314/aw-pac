#[doc = "Register `pg_cfg0` reader"]
pub struct R(crate::R<PG_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PG_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PG_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PG_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pg_cfg0` writer"]
pub struct W(crate::W<PG_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PG_CFG0_SPEC>;
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
impl From<crate::W<PG_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PG_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pg0_select` reader - PG0 Select"]
pub type PG0_SELECT_R = crate::FieldReader<u8, PG0_SELECT_A>;
#[doc = "PG0 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PG0_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    SDC1_CLK = 2,
    #[doc = "4: `100`"]
    RGMII_RXCTRL_RMII_CRS_DV = 4,
    #[doc = "14: `1110`"]
    PG_EINT0 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    UART3_TX = 3,
    #[doc = "5: `101`"]
    PWM7 = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PG0_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PG0_SELECT_A) -> Self {
        variant as _
    }
}
impl PG0_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PG0_SELECT_A> {
        match self.bits {
            0 => Some(PG0_SELECT_A::INPUT),
            2 => Some(PG0_SELECT_A::SDC1_CLK),
            4 => Some(PG0_SELECT_A::RGMII_RXCTRL_RMII_CRS_DV),
            14 => Some(PG0_SELECT_A::PG_EINT0),
            1 => Some(PG0_SELECT_A::OUTPUT),
            3 => Some(PG0_SELECT_A::UART3_TX),
            5 => Some(PG0_SELECT_A::PWM7),
            15 => Some(PG0_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PG0_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `SDC1_CLK`"]
    #[inline(always)]
    pub fn is_sdc1_clk(&self) -> bool {
        *self == PG0_SELECT_A::SDC1_CLK
    }
    #[doc = "Checks if the value of the field is `RGMII_RXCTRL_RMII_CRS_DV`"]
    #[inline(always)]
    pub fn is_rgmii_rxctrl_rmii_crs_dv(&self) -> bool {
        *self == PG0_SELECT_A::RGMII_RXCTRL_RMII_CRS_DV
    }
    #[doc = "Checks if the value of the field is `PG_EINT0`"]
    #[inline(always)]
    pub fn is_pg_eint0(&self) -> bool {
        *self == PG0_SELECT_A::PG_EINT0
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PG0_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `UART3_TX`"]
    #[inline(always)]
    pub fn is_uart3_tx(&self) -> bool {
        *self == PG0_SELECT_A::UART3_TX
    }
    #[doc = "Checks if the value of the field is `PWM7`"]
    #[inline(always)]
    pub fn is_pwm7(&self) -> bool {
        *self == PG0_SELECT_A::PWM7
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PG0_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pg0_select` writer - PG0 Select"]
pub type PG0_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PG_CFG0_SPEC, u8, PG0_SELECT_A, 4, O>;
impl<'a, const O: u8> PG0_SELECT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PG0_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sdc1_clk(self) -> &'a mut W {
        self.variant(PG0_SELECT_A::SDC1_CLK)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn rgmii_rxctrl_rmii_crs_dv(self) -> &'a mut W {
        self.variant(PG0_SELECT_A::RGMII_RXCTRL_RMII_CRS_DV)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pg_eint0(self) -> &'a mut W {
        self.variant(PG0_SELECT_A::PG_EINT0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PG0_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn uart3_tx(self) -> &'a mut W {
        self.variant(PG0_SELECT_A::UART3_TX)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn pwm7(self) -> &'a mut W {
        self.variant(PG0_SELECT_A::PWM7)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PG0_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pg1_select` reader - PG1 Select"]
pub type PG1_SELECT_R = crate::FieldReader<u8, PG1_SELECT_A>;
#[doc = "PG1 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PG1_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    SDC1_CMD = 2,
    #[doc = "4: `100`"]
    RGMII_RXD0_RMII_RXD0 = 4,
    #[doc = "14: `1110`"]
    PG_EINT1 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    UART3_RX = 3,
    #[doc = "5: `101`"]
    PWM6 = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PG1_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PG1_SELECT_A) -> Self {
        variant as _
    }
}
impl PG1_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PG1_SELECT_A> {
        match self.bits {
            0 => Some(PG1_SELECT_A::INPUT),
            2 => Some(PG1_SELECT_A::SDC1_CMD),
            4 => Some(PG1_SELECT_A::RGMII_RXD0_RMII_RXD0),
            14 => Some(PG1_SELECT_A::PG_EINT1),
            1 => Some(PG1_SELECT_A::OUTPUT),
            3 => Some(PG1_SELECT_A::UART3_RX),
            5 => Some(PG1_SELECT_A::PWM6),
            15 => Some(PG1_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PG1_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `SDC1_CMD`"]
    #[inline(always)]
    pub fn is_sdc1_cmd(&self) -> bool {
        *self == PG1_SELECT_A::SDC1_CMD
    }
    #[doc = "Checks if the value of the field is `RGMII_RXD0_RMII_RXD0`"]
    #[inline(always)]
    pub fn is_rgmii_rxd0_rmii_rxd0(&self) -> bool {
        *self == PG1_SELECT_A::RGMII_RXD0_RMII_RXD0
    }
    #[doc = "Checks if the value of the field is `PG_EINT1`"]
    #[inline(always)]
    pub fn is_pg_eint1(&self) -> bool {
        *self == PG1_SELECT_A::PG_EINT1
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PG1_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `UART3_RX`"]
    #[inline(always)]
    pub fn is_uart3_rx(&self) -> bool {
        *self == PG1_SELECT_A::UART3_RX
    }
    #[doc = "Checks if the value of the field is `PWM6`"]
    #[inline(always)]
    pub fn is_pwm6(&self) -> bool {
        *self == PG1_SELECT_A::PWM6
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PG1_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pg1_select` writer - PG1 Select"]
pub type PG1_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PG_CFG0_SPEC, u8, PG1_SELECT_A, 4, O>;
impl<'a, const O: u8> PG1_SELECT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PG1_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sdc1_cmd(self) -> &'a mut W {
        self.variant(PG1_SELECT_A::SDC1_CMD)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn rgmii_rxd0_rmii_rxd0(self) -> &'a mut W {
        self.variant(PG1_SELECT_A::RGMII_RXD0_RMII_RXD0)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pg_eint1(self) -> &'a mut W {
        self.variant(PG1_SELECT_A::PG_EINT1)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PG1_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn uart3_rx(self) -> &'a mut W {
        self.variant(PG1_SELECT_A::UART3_RX)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn pwm6(self) -> &'a mut W {
        self.variant(PG1_SELECT_A::PWM6)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PG1_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pg2_select` reader - PG2 Select"]
pub type PG2_SELECT_R = crate::FieldReader<u8, PG2_SELECT_A>;
#[doc = "PG2 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PG2_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    SDC1_D0 = 2,
    #[doc = "4: `100`"]
    RGMII_RXD1_RMII_RXD1 = 4,
    #[doc = "14: `1110`"]
    PG_EINT2 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    UART3_RTS = 3,
    #[doc = "5: `101`"]
    UART4_TX = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PG2_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PG2_SELECT_A) -> Self {
        variant as _
    }
}
impl PG2_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PG2_SELECT_A> {
        match self.bits {
            0 => Some(PG2_SELECT_A::INPUT),
            2 => Some(PG2_SELECT_A::SDC1_D0),
            4 => Some(PG2_SELECT_A::RGMII_RXD1_RMII_RXD1),
            14 => Some(PG2_SELECT_A::PG_EINT2),
            1 => Some(PG2_SELECT_A::OUTPUT),
            3 => Some(PG2_SELECT_A::UART3_RTS),
            5 => Some(PG2_SELECT_A::UART4_TX),
            15 => Some(PG2_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PG2_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `SDC1_D0`"]
    #[inline(always)]
    pub fn is_sdc1_d0(&self) -> bool {
        *self == PG2_SELECT_A::SDC1_D0
    }
    #[doc = "Checks if the value of the field is `RGMII_RXD1_RMII_RXD1`"]
    #[inline(always)]
    pub fn is_rgmii_rxd1_rmii_rxd1(&self) -> bool {
        *self == PG2_SELECT_A::RGMII_RXD1_RMII_RXD1
    }
    #[doc = "Checks if the value of the field is `PG_EINT2`"]
    #[inline(always)]
    pub fn is_pg_eint2(&self) -> bool {
        *self == PG2_SELECT_A::PG_EINT2
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PG2_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `UART3_RTS`"]
    #[inline(always)]
    pub fn is_uart3_rts(&self) -> bool {
        *self == PG2_SELECT_A::UART3_RTS
    }
    #[doc = "Checks if the value of the field is `UART4_TX`"]
    #[inline(always)]
    pub fn is_uart4_tx(&self) -> bool {
        *self == PG2_SELECT_A::UART4_TX
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PG2_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pg2_select` writer - PG2 Select"]
pub type PG2_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PG_CFG0_SPEC, u8, PG2_SELECT_A, 4, O>;
impl<'a, const O: u8> PG2_SELECT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PG2_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sdc1_d0(self) -> &'a mut W {
        self.variant(PG2_SELECT_A::SDC1_D0)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn rgmii_rxd1_rmii_rxd1(self) -> &'a mut W {
        self.variant(PG2_SELECT_A::RGMII_RXD1_RMII_RXD1)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pg_eint2(self) -> &'a mut W {
        self.variant(PG2_SELECT_A::PG_EINT2)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PG2_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn uart3_rts(self) -> &'a mut W {
        self.variant(PG2_SELECT_A::UART3_RTS)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn uart4_tx(self) -> &'a mut W {
        self.variant(PG2_SELECT_A::UART4_TX)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PG2_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pg3_select` reader - PG3 Select"]
pub type PG3_SELECT_R = crate::FieldReader<u8, PG3_SELECT_A>;
#[doc = "PG3 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PG3_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    SDC1_D1 = 2,
    #[doc = "4: `100`"]
    RGMII_TXCK_RMII_TXCK = 4,
    #[doc = "14: `1110`"]
    PG_EINT3 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    UART3_CTS = 3,
    #[doc = "5: `101`"]
    UART4_RX = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PG3_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PG3_SELECT_A) -> Self {
        variant as _
    }
}
impl PG3_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PG3_SELECT_A> {
        match self.bits {
            0 => Some(PG3_SELECT_A::INPUT),
            2 => Some(PG3_SELECT_A::SDC1_D1),
            4 => Some(PG3_SELECT_A::RGMII_TXCK_RMII_TXCK),
            14 => Some(PG3_SELECT_A::PG_EINT3),
            1 => Some(PG3_SELECT_A::OUTPUT),
            3 => Some(PG3_SELECT_A::UART3_CTS),
            5 => Some(PG3_SELECT_A::UART4_RX),
            15 => Some(PG3_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PG3_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `SDC1_D1`"]
    #[inline(always)]
    pub fn is_sdc1_d1(&self) -> bool {
        *self == PG3_SELECT_A::SDC1_D1
    }
    #[doc = "Checks if the value of the field is `RGMII_TXCK_RMII_TXCK`"]
    #[inline(always)]
    pub fn is_rgmii_txck_rmii_txck(&self) -> bool {
        *self == PG3_SELECT_A::RGMII_TXCK_RMII_TXCK
    }
    #[doc = "Checks if the value of the field is `PG_EINT3`"]
    #[inline(always)]
    pub fn is_pg_eint3(&self) -> bool {
        *self == PG3_SELECT_A::PG_EINT3
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PG3_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `UART3_CTS`"]
    #[inline(always)]
    pub fn is_uart3_cts(&self) -> bool {
        *self == PG3_SELECT_A::UART3_CTS
    }
    #[doc = "Checks if the value of the field is `UART4_RX`"]
    #[inline(always)]
    pub fn is_uart4_rx(&self) -> bool {
        *self == PG3_SELECT_A::UART4_RX
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PG3_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pg3_select` writer - PG3 Select"]
pub type PG3_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PG_CFG0_SPEC, u8, PG3_SELECT_A, 4, O>;
impl<'a, const O: u8> PG3_SELECT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PG3_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sdc1_d1(self) -> &'a mut W {
        self.variant(PG3_SELECT_A::SDC1_D1)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn rgmii_txck_rmii_txck(self) -> &'a mut W {
        self.variant(PG3_SELECT_A::RGMII_TXCK_RMII_TXCK)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pg_eint3(self) -> &'a mut W {
        self.variant(PG3_SELECT_A::PG_EINT3)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PG3_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn uart3_cts(self) -> &'a mut W {
        self.variant(PG3_SELECT_A::UART3_CTS)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn uart4_rx(self) -> &'a mut W {
        self.variant(PG3_SELECT_A::UART4_RX)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PG3_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pg4_select` reader - PG4 Select"]
pub type PG4_SELECT_R = crate::FieldReader<u8, PG4_SELECT_A>;
#[doc = "PG4 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PG4_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    SDC1_D2 = 2,
    #[doc = "4: `100`"]
    RGMII_TXD0_RMII_TXD0 = 4,
    #[doc = "14: `1110`"]
    PG_EINT4 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    UART5_TX = 3,
    #[doc = "5: `101`"]
    PWM5 = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PG4_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PG4_SELECT_A) -> Self {
        variant as _
    }
}
impl PG4_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PG4_SELECT_A> {
        match self.bits {
            0 => Some(PG4_SELECT_A::INPUT),
            2 => Some(PG4_SELECT_A::SDC1_D2),
            4 => Some(PG4_SELECT_A::RGMII_TXD0_RMII_TXD0),
            14 => Some(PG4_SELECT_A::PG_EINT4),
            1 => Some(PG4_SELECT_A::OUTPUT),
            3 => Some(PG4_SELECT_A::UART5_TX),
            5 => Some(PG4_SELECT_A::PWM5),
            15 => Some(PG4_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PG4_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `SDC1_D2`"]
    #[inline(always)]
    pub fn is_sdc1_d2(&self) -> bool {
        *self == PG4_SELECT_A::SDC1_D2
    }
    #[doc = "Checks if the value of the field is `RGMII_TXD0_RMII_TXD0`"]
    #[inline(always)]
    pub fn is_rgmii_txd0_rmii_txd0(&self) -> bool {
        *self == PG4_SELECT_A::RGMII_TXD0_RMII_TXD0
    }
    #[doc = "Checks if the value of the field is `PG_EINT4`"]
    #[inline(always)]
    pub fn is_pg_eint4(&self) -> bool {
        *self == PG4_SELECT_A::PG_EINT4
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PG4_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `UART5_TX`"]
    #[inline(always)]
    pub fn is_uart5_tx(&self) -> bool {
        *self == PG4_SELECT_A::UART5_TX
    }
    #[doc = "Checks if the value of the field is `PWM5`"]
    #[inline(always)]
    pub fn is_pwm5(&self) -> bool {
        *self == PG4_SELECT_A::PWM5
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PG4_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pg4_select` writer - PG4 Select"]
pub type PG4_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PG_CFG0_SPEC, u8, PG4_SELECT_A, 4, O>;
impl<'a, const O: u8> PG4_SELECT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PG4_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sdc1_d2(self) -> &'a mut W {
        self.variant(PG4_SELECT_A::SDC1_D2)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn rgmii_txd0_rmii_txd0(self) -> &'a mut W {
        self.variant(PG4_SELECT_A::RGMII_TXD0_RMII_TXD0)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pg_eint4(self) -> &'a mut W {
        self.variant(PG4_SELECT_A::PG_EINT4)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PG4_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn uart5_tx(self) -> &'a mut W {
        self.variant(PG4_SELECT_A::UART5_TX)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn pwm5(self) -> &'a mut W {
        self.variant(PG4_SELECT_A::PWM5)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PG4_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pg5_select` reader - PG5 Select"]
pub type PG5_SELECT_R = crate::FieldReader<u8, PG5_SELECT_A>;
#[doc = "PG5 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PG5_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    SDC1_D3 = 2,
    #[doc = "4: `100`"]
    RGMII_TXD1_RMII_TXD1 = 4,
    #[doc = "14: `1110`"]
    PG_EINT5 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    UART5_RX = 3,
    #[doc = "5: `101`"]
    PWM4 = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PG5_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PG5_SELECT_A) -> Self {
        variant as _
    }
}
impl PG5_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PG5_SELECT_A> {
        match self.bits {
            0 => Some(PG5_SELECT_A::INPUT),
            2 => Some(PG5_SELECT_A::SDC1_D3),
            4 => Some(PG5_SELECT_A::RGMII_TXD1_RMII_TXD1),
            14 => Some(PG5_SELECT_A::PG_EINT5),
            1 => Some(PG5_SELECT_A::OUTPUT),
            3 => Some(PG5_SELECT_A::UART5_RX),
            5 => Some(PG5_SELECT_A::PWM4),
            15 => Some(PG5_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PG5_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `SDC1_D3`"]
    #[inline(always)]
    pub fn is_sdc1_d3(&self) -> bool {
        *self == PG5_SELECT_A::SDC1_D3
    }
    #[doc = "Checks if the value of the field is `RGMII_TXD1_RMII_TXD1`"]
    #[inline(always)]
    pub fn is_rgmii_txd1_rmii_txd1(&self) -> bool {
        *self == PG5_SELECT_A::RGMII_TXD1_RMII_TXD1
    }
    #[doc = "Checks if the value of the field is `PG_EINT5`"]
    #[inline(always)]
    pub fn is_pg_eint5(&self) -> bool {
        *self == PG5_SELECT_A::PG_EINT5
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PG5_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `UART5_RX`"]
    #[inline(always)]
    pub fn is_uart5_rx(&self) -> bool {
        *self == PG5_SELECT_A::UART5_RX
    }
    #[doc = "Checks if the value of the field is `PWM4`"]
    #[inline(always)]
    pub fn is_pwm4(&self) -> bool {
        *self == PG5_SELECT_A::PWM4
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PG5_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pg5_select` writer - PG5 Select"]
pub type PG5_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PG_CFG0_SPEC, u8, PG5_SELECT_A, 4, O>;
impl<'a, const O: u8> PG5_SELECT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PG5_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sdc1_d3(self) -> &'a mut W {
        self.variant(PG5_SELECT_A::SDC1_D3)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn rgmii_txd1_rmii_txd1(self) -> &'a mut W {
        self.variant(PG5_SELECT_A::RGMII_TXD1_RMII_TXD1)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pg_eint5(self) -> &'a mut W {
        self.variant(PG5_SELECT_A::PG_EINT5)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PG5_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn uart5_rx(self) -> &'a mut W {
        self.variant(PG5_SELECT_A::UART5_RX)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn pwm4(self) -> &'a mut W {
        self.variant(PG5_SELECT_A::PWM4)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PG5_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pg6_select` reader - PG6 Select"]
pub type PG6_SELECT_R = crate::FieldReader<u8, PG6_SELECT_A>;
#[doc = "PG6 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PG6_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    UART1_TX = 2,
    #[doc = "4: `100`"]
    RGMII_TXD2 = 4,
    #[doc = "14: `1110`"]
    PG_EINT6 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    TWI2_SCK = 3,
    #[doc = "5: `101`"]
    PWM1 = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PG6_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PG6_SELECT_A) -> Self {
        variant as _
    }
}
impl PG6_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PG6_SELECT_A> {
        match self.bits {
            0 => Some(PG6_SELECT_A::INPUT),
            2 => Some(PG6_SELECT_A::UART1_TX),
            4 => Some(PG6_SELECT_A::RGMII_TXD2),
            14 => Some(PG6_SELECT_A::PG_EINT6),
            1 => Some(PG6_SELECT_A::OUTPUT),
            3 => Some(PG6_SELECT_A::TWI2_SCK),
            5 => Some(PG6_SELECT_A::PWM1),
            15 => Some(PG6_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PG6_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `UART1_TX`"]
    #[inline(always)]
    pub fn is_uart1_tx(&self) -> bool {
        *self == PG6_SELECT_A::UART1_TX
    }
    #[doc = "Checks if the value of the field is `RGMII_TXD2`"]
    #[inline(always)]
    pub fn is_rgmii_txd2(&self) -> bool {
        *self == PG6_SELECT_A::RGMII_TXD2
    }
    #[doc = "Checks if the value of the field is `PG_EINT6`"]
    #[inline(always)]
    pub fn is_pg_eint6(&self) -> bool {
        *self == PG6_SELECT_A::PG_EINT6
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PG6_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `TWI2_SCK`"]
    #[inline(always)]
    pub fn is_twi2_sck(&self) -> bool {
        *self == PG6_SELECT_A::TWI2_SCK
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == PG6_SELECT_A::PWM1
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PG6_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pg6_select` writer - PG6 Select"]
pub type PG6_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PG_CFG0_SPEC, u8, PG6_SELECT_A, 4, O>;
impl<'a, const O: u8> PG6_SELECT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PG6_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn uart1_tx(self) -> &'a mut W {
        self.variant(PG6_SELECT_A::UART1_TX)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn rgmii_txd2(self) -> &'a mut W {
        self.variant(PG6_SELECT_A::RGMII_TXD2)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pg_eint6(self) -> &'a mut W {
        self.variant(PG6_SELECT_A::PG_EINT6)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PG6_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn twi2_sck(self) -> &'a mut W {
        self.variant(PG6_SELECT_A::TWI2_SCK)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(PG6_SELECT_A::PWM1)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PG6_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pg7_select` reader - PG7 Select"]
pub type PG7_SELECT_R = crate::FieldReader<u8, PG7_SELECT_A>;
#[doc = "PG7 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PG7_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    UART1_RX = 2,
    #[doc = "4: `100`"]
    RGMII_TXD3 = 4,
    #[doc = "14: `1110`"]
    PG_EINT7 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    TWI2_SDA = 3,
    #[doc = "5: `101`"]
    OWA_IN = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PG7_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PG7_SELECT_A) -> Self {
        variant as _
    }
}
impl PG7_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PG7_SELECT_A> {
        match self.bits {
            0 => Some(PG7_SELECT_A::INPUT),
            2 => Some(PG7_SELECT_A::UART1_RX),
            4 => Some(PG7_SELECT_A::RGMII_TXD3),
            14 => Some(PG7_SELECT_A::PG_EINT7),
            1 => Some(PG7_SELECT_A::OUTPUT),
            3 => Some(PG7_SELECT_A::TWI2_SDA),
            5 => Some(PG7_SELECT_A::OWA_IN),
            15 => Some(PG7_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PG7_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `UART1_RX`"]
    #[inline(always)]
    pub fn is_uart1_rx(&self) -> bool {
        *self == PG7_SELECT_A::UART1_RX
    }
    #[doc = "Checks if the value of the field is `RGMII_TXD3`"]
    #[inline(always)]
    pub fn is_rgmii_txd3(&self) -> bool {
        *self == PG7_SELECT_A::RGMII_TXD3
    }
    #[doc = "Checks if the value of the field is `PG_EINT7`"]
    #[inline(always)]
    pub fn is_pg_eint7(&self) -> bool {
        *self == PG7_SELECT_A::PG_EINT7
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PG7_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `TWI2_SDA`"]
    #[inline(always)]
    pub fn is_twi2_sda(&self) -> bool {
        *self == PG7_SELECT_A::TWI2_SDA
    }
    #[doc = "Checks if the value of the field is `OWA_IN`"]
    #[inline(always)]
    pub fn is_owa_in(&self) -> bool {
        *self == PG7_SELECT_A::OWA_IN
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PG7_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pg7_select` writer - PG7 Select"]
pub type PG7_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PG_CFG0_SPEC, u8, PG7_SELECT_A, 4, O>;
impl<'a, const O: u8> PG7_SELECT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PG7_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn uart1_rx(self) -> &'a mut W {
        self.variant(PG7_SELECT_A::UART1_RX)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn rgmii_txd3(self) -> &'a mut W {
        self.variant(PG7_SELECT_A::RGMII_TXD3)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pg_eint7(self) -> &'a mut W {
        self.variant(PG7_SELECT_A::PG_EINT7)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PG7_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn twi2_sda(self) -> &'a mut W {
        self.variant(PG7_SELECT_A::TWI2_SDA)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn owa_in(self) -> &'a mut W {
        self.variant(PG7_SELECT_A::OWA_IN)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PG7_SELECT_A::IO_DISABLE)
    }
}
impl R {
    #[doc = "Bits 0:3 - PG0 Select"]
    #[inline(always)]
    pub fn pg0_select(&self) -> PG0_SELECT_R {
        PG0_SELECT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PG1 Select"]
    #[inline(always)]
    pub fn pg1_select(&self) -> PG1_SELECT_R {
        PG1_SELECT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PG2 Select"]
    #[inline(always)]
    pub fn pg2_select(&self) -> PG2_SELECT_R {
        PG2_SELECT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PG3 Select"]
    #[inline(always)]
    pub fn pg3_select(&self) -> PG3_SELECT_R {
        PG3_SELECT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PG4 Select"]
    #[inline(always)]
    pub fn pg4_select(&self) -> PG4_SELECT_R {
        PG4_SELECT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PG5 Select"]
    #[inline(always)]
    pub fn pg5_select(&self) -> PG5_SELECT_R {
        PG5_SELECT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PG6 Select"]
    #[inline(always)]
    pub fn pg6_select(&self) -> PG6_SELECT_R {
        PG6_SELECT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - PG7 Select"]
    #[inline(always)]
    pub fn pg7_select(&self) -> PG7_SELECT_R {
        PG7_SELECT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PG0 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pg0_select(&mut self) -> PG0_SELECT_W<0> {
        PG0_SELECT_W::new(self)
    }
    #[doc = "Bits 4:7 - PG1 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pg1_select(&mut self) -> PG1_SELECT_W<4> {
        PG1_SELECT_W::new(self)
    }
    #[doc = "Bits 8:11 - PG2 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pg2_select(&mut self) -> PG2_SELECT_W<8> {
        PG2_SELECT_W::new(self)
    }
    #[doc = "Bits 12:15 - PG3 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pg3_select(&mut self) -> PG3_SELECT_W<12> {
        PG3_SELECT_W::new(self)
    }
    #[doc = "Bits 16:19 - PG4 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pg4_select(&mut self) -> PG4_SELECT_W<16> {
        PG4_SELECT_W::new(self)
    }
    #[doc = "Bits 20:23 - PG5 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pg5_select(&mut self) -> PG5_SELECT_W<20> {
        PG5_SELECT_W::new(self)
    }
    #[doc = "Bits 24:27 - PG6 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pg6_select(&mut self) -> PG6_SELECT_W<24> {
        PG6_SELECT_W::new(self)
    }
    #[doc = "Bits 28:31 - PG7 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pg7_select(&mut self) -> PG7_SELECT_W<28> {
        PG7_SELECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PG Configure Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pg_cfg0](index.html) module"]
pub struct PG_CFG0_SPEC;
impl crate::RegisterSpec for PG_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pg_cfg0::R](R) reader structure"]
impl crate::Readable for PG_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pg_cfg0::W](W) writer structure"]
impl crate::Writable for PG_CFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pg_cfg0 to value 0"]
impl crate::Resettable for PG_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
