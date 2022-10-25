#[doc = "Register `pe_cfg0` reader"]
pub struct R(crate::R<PE_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PE_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PE_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PE_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pe_cfg0` writer"]
pub struct W(crate::W<PE_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PE_CFG0_SPEC>;
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
impl From<crate::W<PE_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PE_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pe0_select` reader - PE0 Select"]
pub type PE0_SELECT_R = crate::FieldReader<u8, PE0_SELECT_A>;
#[doc = "PE0 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PE0_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    NCSI0_HSYNC = 2,
    #[doc = "4: `100`"]
    TWI1_SCK = 4,
    #[doc = "8: `1000`"]
    RGMII_RXCTRL_RMII_CRS_DV = 8,
    #[doc = "14: `1110`"]
    PE_EINT0 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    UART2_RTS = 3,
    #[doc = "5: `101`"]
    LCD0_HSYNC = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PE0_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PE0_SELECT_A) -> Self {
        variant as _
    }
}
impl PE0_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PE0_SELECT_A> {
        match self.bits {
            0 => Some(PE0_SELECT_A::INPUT),
            2 => Some(PE0_SELECT_A::NCSI0_HSYNC),
            4 => Some(PE0_SELECT_A::TWI1_SCK),
            8 => Some(PE0_SELECT_A::RGMII_RXCTRL_RMII_CRS_DV),
            14 => Some(PE0_SELECT_A::PE_EINT0),
            1 => Some(PE0_SELECT_A::OUTPUT),
            3 => Some(PE0_SELECT_A::UART2_RTS),
            5 => Some(PE0_SELECT_A::LCD0_HSYNC),
            15 => Some(PE0_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PE0_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `NCSI0_HSYNC`"]
    #[inline(always)]
    pub fn is_ncsi0_hsync(&self) -> bool {
        *self == PE0_SELECT_A::NCSI0_HSYNC
    }
    #[doc = "Checks if the value of the field is `TWI1_SCK`"]
    #[inline(always)]
    pub fn is_twi1_sck(&self) -> bool {
        *self == PE0_SELECT_A::TWI1_SCK
    }
    #[doc = "Checks if the value of the field is `RGMII_RXCTRL_RMII_CRS_DV`"]
    #[inline(always)]
    pub fn is_rgmii_rxctrl_rmii_crs_dv(&self) -> bool {
        *self == PE0_SELECT_A::RGMII_RXCTRL_RMII_CRS_DV
    }
    #[doc = "Checks if the value of the field is `PE_EINT0`"]
    #[inline(always)]
    pub fn is_pe_eint0(&self) -> bool {
        *self == PE0_SELECT_A::PE_EINT0
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PE0_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `UART2_RTS`"]
    #[inline(always)]
    pub fn is_uart2_rts(&self) -> bool {
        *self == PE0_SELECT_A::UART2_RTS
    }
    #[doc = "Checks if the value of the field is `LCD0_HSYNC`"]
    #[inline(always)]
    pub fn is_lcd0_hsync(&self) -> bool {
        *self == PE0_SELECT_A::LCD0_HSYNC
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PE0_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pe0_select` writer - PE0 Select"]
pub type PE0_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PE_CFG0_SPEC, u8, PE0_SELECT_A, 4, O>;
impl<'a, const O: u8> PE0_SELECT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PE0_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn ncsi0_hsync(self) -> &'a mut W {
        self.variant(PE0_SELECT_A::NCSI0_HSYNC)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn twi1_sck(self) -> &'a mut W {
        self.variant(PE0_SELECT_A::TWI1_SCK)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn rgmii_rxctrl_rmii_crs_dv(self) -> &'a mut W {
        self.variant(PE0_SELECT_A::RGMII_RXCTRL_RMII_CRS_DV)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pe_eint0(self) -> &'a mut W {
        self.variant(PE0_SELECT_A::PE_EINT0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PE0_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn uart2_rts(self) -> &'a mut W {
        self.variant(PE0_SELECT_A::UART2_RTS)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn lcd0_hsync(self) -> &'a mut W {
        self.variant(PE0_SELECT_A::LCD0_HSYNC)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PE0_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pe1_select` reader - PE1 Select"]
pub type PE1_SELECT_R = crate::FieldReader<u8, PE1_SELECT_A>;
#[doc = "PE1 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PE1_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    NCSI0_VSYNC = 2,
    #[doc = "4: `100`"]
    TWI1_SDA = 4,
    #[doc = "8: `1000`"]
    RGMII_RXD0_RMII_RXD0 = 8,
    #[doc = "14: `1110`"]
    PE_EINT1 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    UART2_CTS = 3,
    #[doc = "5: `101`"]
    LCD0_VSYNC = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PE1_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PE1_SELECT_A) -> Self {
        variant as _
    }
}
impl PE1_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PE1_SELECT_A> {
        match self.bits {
            0 => Some(PE1_SELECT_A::INPUT),
            2 => Some(PE1_SELECT_A::NCSI0_VSYNC),
            4 => Some(PE1_SELECT_A::TWI1_SDA),
            8 => Some(PE1_SELECT_A::RGMII_RXD0_RMII_RXD0),
            14 => Some(PE1_SELECT_A::PE_EINT1),
            1 => Some(PE1_SELECT_A::OUTPUT),
            3 => Some(PE1_SELECT_A::UART2_CTS),
            5 => Some(PE1_SELECT_A::LCD0_VSYNC),
            15 => Some(PE1_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PE1_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `NCSI0_VSYNC`"]
    #[inline(always)]
    pub fn is_ncsi0_vsync(&self) -> bool {
        *self == PE1_SELECT_A::NCSI0_VSYNC
    }
    #[doc = "Checks if the value of the field is `TWI1_SDA`"]
    #[inline(always)]
    pub fn is_twi1_sda(&self) -> bool {
        *self == PE1_SELECT_A::TWI1_SDA
    }
    #[doc = "Checks if the value of the field is `RGMII_RXD0_RMII_RXD0`"]
    #[inline(always)]
    pub fn is_rgmii_rxd0_rmii_rxd0(&self) -> bool {
        *self == PE1_SELECT_A::RGMII_RXD0_RMII_RXD0
    }
    #[doc = "Checks if the value of the field is `PE_EINT1`"]
    #[inline(always)]
    pub fn is_pe_eint1(&self) -> bool {
        *self == PE1_SELECT_A::PE_EINT1
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PE1_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `UART2_CTS`"]
    #[inline(always)]
    pub fn is_uart2_cts(&self) -> bool {
        *self == PE1_SELECT_A::UART2_CTS
    }
    #[doc = "Checks if the value of the field is `LCD0_VSYNC`"]
    #[inline(always)]
    pub fn is_lcd0_vsync(&self) -> bool {
        *self == PE1_SELECT_A::LCD0_VSYNC
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PE1_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pe1_select` writer - PE1 Select"]
pub type PE1_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PE_CFG0_SPEC, u8, PE1_SELECT_A, 4, O>;
impl<'a, const O: u8> PE1_SELECT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PE1_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn ncsi0_vsync(self) -> &'a mut W {
        self.variant(PE1_SELECT_A::NCSI0_VSYNC)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn twi1_sda(self) -> &'a mut W {
        self.variant(PE1_SELECT_A::TWI1_SDA)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn rgmii_rxd0_rmii_rxd0(self) -> &'a mut W {
        self.variant(PE1_SELECT_A::RGMII_RXD0_RMII_RXD0)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pe_eint1(self) -> &'a mut W {
        self.variant(PE1_SELECT_A::PE_EINT1)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PE1_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn uart2_cts(self) -> &'a mut W {
        self.variant(PE1_SELECT_A::UART2_CTS)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn lcd0_vsync(self) -> &'a mut W {
        self.variant(PE1_SELECT_A::LCD0_VSYNC)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PE1_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pe2_select` reader - PE2 Select"]
pub type PE2_SELECT_R = crate::FieldReader<u8, PE2_SELECT_A>;
#[doc = "PE2 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PE2_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    NCSI0_PCLK = 2,
    #[doc = "4: `100`"]
    TWI0_SCK = 4,
    #[doc = "6: `110`"]
    UART0_TX = 6,
    #[doc = "8: `1000`"]
    RGMII_RXD1_RMII_RXD1 = 8,
    #[doc = "14: `1110`"]
    PE_EINT2 = 14,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    UART2_TX = 3,
    #[doc = "5: `101`"]
    CLK_FANOUT0 = 5,
}
impl From<PE2_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PE2_SELECT_A) -> Self {
        variant as _
    }
}
impl PE2_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PE2_SELECT_A> {
        match self.bits {
            0 => Some(PE2_SELECT_A::INPUT),
            2 => Some(PE2_SELECT_A::NCSI0_PCLK),
            4 => Some(PE2_SELECT_A::TWI0_SCK),
            6 => Some(PE2_SELECT_A::UART0_TX),
            8 => Some(PE2_SELECT_A::RGMII_RXD1_RMII_RXD1),
            14 => Some(PE2_SELECT_A::PE_EINT2),
            15 => Some(PE2_SELECT_A::IO_DISABLE),
            1 => Some(PE2_SELECT_A::OUTPUT),
            3 => Some(PE2_SELECT_A::UART2_TX),
            5 => Some(PE2_SELECT_A::CLK_FANOUT0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PE2_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `NCSI0_PCLK`"]
    #[inline(always)]
    pub fn is_ncsi0_pclk(&self) -> bool {
        *self == PE2_SELECT_A::NCSI0_PCLK
    }
    #[doc = "Checks if the value of the field is `TWI0_SCK`"]
    #[inline(always)]
    pub fn is_twi0_sck(&self) -> bool {
        *self == PE2_SELECT_A::TWI0_SCK
    }
    #[doc = "Checks if the value of the field is `UART0_TX`"]
    #[inline(always)]
    pub fn is_uart0_tx(&self) -> bool {
        *self == PE2_SELECT_A::UART0_TX
    }
    #[doc = "Checks if the value of the field is `RGMII_RXD1_RMII_RXD1`"]
    #[inline(always)]
    pub fn is_rgmii_rxd1_rmii_rxd1(&self) -> bool {
        *self == PE2_SELECT_A::RGMII_RXD1_RMII_RXD1
    }
    #[doc = "Checks if the value of the field is `PE_EINT2`"]
    #[inline(always)]
    pub fn is_pe_eint2(&self) -> bool {
        *self == PE2_SELECT_A::PE_EINT2
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PE2_SELECT_A::IO_DISABLE
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PE2_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `UART2_TX`"]
    #[inline(always)]
    pub fn is_uart2_tx(&self) -> bool {
        *self == PE2_SELECT_A::UART2_TX
    }
    #[doc = "Checks if the value of the field is `CLK_FANOUT0`"]
    #[inline(always)]
    pub fn is_clk_fanout0(&self) -> bool {
        *self == PE2_SELECT_A::CLK_FANOUT0
    }
}
#[doc = "Field `pe2_select` writer - PE2 Select"]
pub type PE2_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PE_CFG0_SPEC, u8, PE2_SELECT_A, 4, O>;
impl<'a, const O: u8> PE2_SELECT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PE2_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn ncsi0_pclk(self) -> &'a mut W {
        self.variant(PE2_SELECT_A::NCSI0_PCLK)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn twi0_sck(self) -> &'a mut W {
        self.variant(PE2_SELECT_A::TWI0_SCK)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn uart0_tx(self) -> &'a mut W {
        self.variant(PE2_SELECT_A::UART0_TX)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn rgmii_rxd1_rmii_rxd1(self) -> &'a mut W {
        self.variant(PE2_SELECT_A::RGMII_RXD1_RMII_RXD1)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pe_eint2(self) -> &'a mut W {
        self.variant(PE2_SELECT_A::PE_EINT2)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PE2_SELECT_A::IO_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PE2_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn uart2_tx(self) -> &'a mut W {
        self.variant(PE2_SELECT_A::UART2_TX)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn clk_fanout0(self) -> &'a mut W {
        self.variant(PE2_SELECT_A::CLK_FANOUT0)
    }
}
#[doc = "Field `pe3_select` reader - PE3 Select"]
pub type PE3_SELECT_R = crate::FieldReader<u8, PE3_SELECT_A>;
#[doc = "PE3 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PE3_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    NCSI0_MCLK = 2,
    #[doc = "4: `100`"]
    TWI0_SDA = 4,
    #[doc = "6: `110`"]
    UART0_RX = 6,
    #[doc = "8: `1000`"]
    RGMII_TXCK_RMII_TXCK = 8,
    #[doc = "14: `1110`"]
    PE_EINT3 = 14,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    UART2_RX = 3,
    #[doc = "5: `101`"]
    CLK_FANOUT1 = 5,
}
impl From<PE3_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PE3_SELECT_A) -> Self {
        variant as _
    }
}
impl PE3_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PE3_SELECT_A> {
        match self.bits {
            0 => Some(PE3_SELECT_A::INPUT),
            2 => Some(PE3_SELECT_A::NCSI0_MCLK),
            4 => Some(PE3_SELECT_A::TWI0_SDA),
            6 => Some(PE3_SELECT_A::UART0_RX),
            8 => Some(PE3_SELECT_A::RGMII_TXCK_RMII_TXCK),
            14 => Some(PE3_SELECT_A::PE_EINT3),
            15 => Some(PE3_SELECT_A::IO_DISABLE),
            1 => Some(PE3_SELECT_A::OUTPUT),
            3 => Some(PE3_SELECT_A::UART2_RX),
            5 => Some(PE3_SELECT_A::CLK_FANOUT1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PE3_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `NCSI0_MCLK`"]
    #[inline(always)]
    pub fn is_ncsi0_mclk(&self) -> bool {
        *self == PE3_SELECT_A::NCSI0_MCLK
    }
    #[doc = "Checks if the value of the field is `TWI0_SDA`"]
    #[inline(always)]
    pub fn is_twi0_sda(&self) -> bool {
        *self == PE3_SELECT_A::TWI0_SDA
    }
    #[doc = "Checks if the value of the field is `UART0_RX`"]
    #[inline(always)]
    pub fn is_uart0_rx(&self) -> bool {
        *self == PE3_SELECT_A::UART0_RX
    }
    #[doc = "Checks if the value of the field is `RGMII_TXCK_RMII_TXCK`"]
    #[inline(always)]
    pub fn is_rgmii_txck_rmii_txck(&self) -> bool {
        *self == PE3_SELECT_A::RGMII_TXCK_RMII_TXCK
    }
    #[doc = "Checks if the value of the field is `PE_EINT3`"]
    #[inline(always)]
    pub fn is_pe_eint3(&self) -> bool {
        *self == PE3_SELECT_A::PE_EINT3
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PE3_SELECT_A::IO_DISABLE
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PE3_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `UART2_RX`"]
    #[inline(always)]
    pub fn is_uart2_rx(&self) -> bool {
        *self == PE3_SELECT_A::UART2_RX
    }
    #[doc = "Checks if the value of the field is `CLK_FANOUT1`"]
    #[inline(always)]
    pub fn is_clk_fanout1(&self) -> bool {
        *self == PE3_SELECT_A::CLK_FANOUT1
    }
}
#[doc = "Field `pe3_select` writer - PE3 Select"]
pub type PE3_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PE_CFG0_SPEC, u8, PE3_SELECT_A, 4, O>;
impl<'a, const O: u8> PE3_SELECT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PE3_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn ncsi0_mclk(self) -> &'a mut W {
        self.variant(PE3_SELECT_A::NCSI0_MCLK)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn twi0_sda(self) -> &'a mut W {
        self.variant(PE3_SELECT_A::TWI0_SDA)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn uart0_rx(self) -> &'a mut W {
        self.variant(PE3_SELECT_A::UART0_RX)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn rgmii_txck_rmii_txck(self) -> &'a mut W {
        self.variant(PE3_SELECT_A::RGMII_TXCK_RMII_TXCK)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pe_eint3(self) -> &'a mut W {
        self.variant(PE3_SELECT_A::PE_EINT3)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PE3_SELECT_A::IO_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PE3_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn uart2_rx(self) -> &'a mut W {
        self.variant(PE3_SELECT_A::UART2_RX)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn clk_fanout1(self) -> &'a mut W {
        self.variant(PE3_SELECT_A::CLK_FANOUT1)
    }
}
#[doc = "Field `pe4_select` reader - PE4 Select"]
pub type PE4_SELECT_R = crate::FieldReader<u8, PE4_SELECT_A>;
#[doc = "PE4 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PE4_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    NCSI0_D0 = 2,
    #[doc = "4: `100`"]
    TWI2_SCK = 4,
    #[doc = "6: `110`"]
    D_JTAG_MS = 6,
    #[doc = "8: `1000`"]
    RGMII_TXD0_RMII_TXD0 = 8,
    #[doc = "14: `1110`"]
    PE_EINT4 = 14,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    UART4_TX = 3,
    #[doc = "5: `101`"]
    CLK_FANOUT2 = 5,
    #[doc = "7: `111`"]
    R_JTAG_MS = 7,
}
impl From<PE4_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PE4_SELECT_A) -> Self {
        variant as _
    }
}
impl PE4_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PE4_SELECT_A> {
        match self.bits {
            0 => Some(PE4_SELECT_A::INPUT),
            2 => Some(PE4_SELECT_A::NCSI0_D0),
            4 => Some(PE4_SELECT_A::TWI2_SCK),
            6 => Some(PE4_SELECT_A::D_JTAG_MS),
            8 => Some(PE4_SELECT_A::RGMII_TXD0_RMII_TXD0),
            14 => Some(PE4_SELECT_A::PE_EINT4),
            15 => Some(PE4_SELECT_A::IO_DISABLE),
            1 => Some(PE4_SELECT_A::OUTPUT),
            3 => Some(PE4_SELECT_A::UART4_TX),
            5 => Some(PE4_SELECT_A::CLK_FANOUT2),
            7 => Some(PE4_SELECT_A::R_JTAG_MS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PE4_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `NCSI0_D0`"]
    #[inline(always)]
    pub fn is_ncsi0_d0(&self) -> bool {
        *self == PE4_SELECT_A::NCSI0_D0
    }
    #[doc = "Checks if the value of the field is `TWI2_SCK`"]
    #[inline(always)]
    pub fn is_twi2_sck(&self) -> bool {
        *self == PE4_SELECT_A::TWI2_SCK
    }
    #[doc = "Checks if the value of the field is `D_JTAG_MS`"]
    #[inline(always)]
    pub fn is_d_jtag_ms(&self) -> bool {
        *self == PE4_SELECT_A::D_JTAG_MS
    }
    #[doc = "Checks if the value of the field is `RGMII_TXD0_RMII_TXD0`"]
    #[inline(always)]
    pub fn is_rgmii_txd0_rmii_txd0(&self) -> bool {
        *self == PE4_SELECT_A::RGMII_TXD0_RMII_TXD0
    }
    #[doc = "Checks if the value of the field is `PE_EINT4`"]
    #[inline(always)]
    pub fn is_pe_eint4(&self) -> bool {
        *self == PE4_SELECT_A::PE_EINT4
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PE4_SELECT_A::IO_DISABLE
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PE4_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `UART4_TX`"]
    #[inline(always)]
    pub fn is_uart4_tx(&self) -> bool {
        *self == PE4_SELECT_A::UART4_TX
    }
    #[doc = "Checks if the value of the field is `CLK_FANOUT2`"]
    #[inline(always)]
    pub fn is_clk_fanout2(&self) -> bool {
        *self == PE4_SELECT_A::CLK_FANOUT2
    }
    #[doc = "Checks if the value of the field is `R_JTAG_MS`"]
    #[inline(always)]
    pub fn is_r_jtag_ms(&self) -> bool {
        *self == PE4_SELECT_A::R_JTAG_MS
    }
}
#[doc = "Field `pe4_select` writer - PE4 Select"]
pub type PE4_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PE_CFG0_SPEC, u8, PE4_SELECT_A, 4, O>;
impl<'a, const O: u8> PE4_SELECT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PE4_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn ncsi0_d0(self) -> &'a mut W {
        self.variant(PE4_SELECT_A::NCSI0_D0)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn twi2_sck(self) -> &'a mut W {
        self.variant(PE4_SELECT_A::TWI2_SCK)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn d_jtag_ms(self) -> &'a mut W {
        self.variant(PE4_SELECT_A::D_JTAG_MS)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn rgmii_txd0_rmii_txd0(self) -> &'a mut W {
        self.variant(PE4_SELECT_A::RGMII_TXD0_RMII_TXD0)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pe_eint4(self) -> &'a mut W {
        self.variant(PE4_SELECT_A::PE_EINT4)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PE4_SELECT_A::IO_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PE4_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn uart4_tx(self) -> &'a mut W {
        self.variant(PE4_SELECT_A::UART4_TX)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn clk_fanout2(self) -> &'a mut W {
        self.variant(PE4_SELECT_A::CLK_FANOUT2)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn r_jtag_ms(self) -> &'a mut W {
        self.variant(PE4_SELECT_A::R_JTAG_MS)
    }
}
#[doc = "Field `pe5_select` reader - PE5 Select"]
pub type PE5_SELECT_R = crate::FieldReader<u8, PE5_SELECT_A>;
#[doc = "PE5 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PE5_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    NCSI0_D1 = 2,
    #[doc = "4: `100`"]
    TWI2_SDA = 4,
    #[doc = "6: `110`"]
    D_JTAG_DI = 6,
    #[doc = "8: `1000`"]
    RGMII_TXD1_RMII_TXD1 = 8,
    #[doc = "14: `1110`"]
    PE_EINT5 = 14,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    UART4_RX = 3,
    #[doc = "5: `101`"]
    LEDC_DO = 5,
    #[doc = "7: `111`"]
    R_JTAG_DI = 7,
}
impl From<PE5_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PE5_SELECT_A) -> Self {
        variant as _
    }
}
impl PE5_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PE5_SELECT_A> {
        match self.bits {
            0 => Some(PE5_SELECT_A::INPUT),
            2 => Some(PE5_SELECT_A::NCSI0_D1),
            4 => Some(PE5_SELECT_A::TWI2_SDA),
            6 => Some(PE5_SELECT_A::D_JTAG_DI),
            8 => Some(PE5_SELECT_A::RGMII_TXD1_RMII_TXD1),
            14 => Some(PE5_SELECT_A::PE_EINT5),
            15 => Some(PE5_SELECT_A::IO_DISABLE),
            1 => Some(PE5_SELECT_A::OUTPUT),
            3 => Some(PE5_SELECT_A::UART4_RX),
            5 => Some(PE5_SELECT_A::LEDC_DO),
            7 => Some(PE5_SELECT_A::R_JTAG_DI),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PE5_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `NCSI0_D1`"]
    #[inline(always)]
    pub fn is_ncsi0_d1(&self) -> bool {
        *self == PE5_SELECT_A::NCSI0_D1
    }
    #[doc = "Checks if the value of the field is `TWI2_SDA`"]
    #[inline(always)]
    pub fn is_twi2_sda(&self) -> bool {
        *self == PE5_SELECT_A::TWI2_SDA
    }
    #[doc = "Checks if the value of the field is `D_JTAG_DI`"]
    #[inline(always)]
    pub fn is_d_jtag_di(&self) -> bool {
        *self == PE5_SELECT_A::D_JTAG_DI
    }
    #[doc = "Checks if the value of the field is `RGMII_TXD1_RMII_TXD1`"]
    #[inline(always)]
    pub fn is_rgmii_txd1_rmii_txd1(&self) -> bool {
        *self == PE5_SELECT_A::RGMII_TXD1_RMII_TXD1
    }
    #[doc = "Checks if the value of the field is `PE_EINT5`"]
    #[inline(always)]
    pub fn is_pe_eint5(&self) -> bool {
        *self == PE5_SELECT_A::PE_EINT5
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PE5_SELECT_A::IO_DISABLE
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PE5_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `UART4_RX`"]
    #[inline(always)]
    pub fn is_uart4_rx(&self) -> bool {
        *self == PE5_SELECT_A::UART4_RX
    }
    #[doc = "Checks if the value of the field is `LEDC_DO`"]
    #[inline(always)]
    pub fn is_ledc_do(&self) -> bool {
        *self == PE5_SELECT_A::LEDC_DO
    }
    #[doc = "Checks if the value of the field is `R_JTAG_DI`"]
    #[inline(always)]
    pub fn is_r_jtag_di(&self) -> bool {
        *self == PE5_SELECT_A::R_JTAG_DI
    }
}
#[doc = "Field `pe5_select` writer - PE5 Select"]
pub type PE5_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PE_CFG0_SPEC, u8, PE5_SELECT_A, 4, O>;
impl<'a, const O: u8> PE5_SELECT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PE5_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn ncsi0_d1(self) -> &'a mut W {
        self.variant(PE5_SELECT_A::NCSI0_D1)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn twi2_sda(self) -> &'a mut W {
        self.variant(PE5_SELECT_A::TWI2_SDA)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn d_jtag_di(self) -> &'a mut W {
        self.variant(PE5_SELECT_A::D_JTAG_DI)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn rgmii_txd1_rmii_txd1(self) -> &'a mut W {
        self.variant(PE5_SELECT_A::RGMII_TXD1_RMII_TXD1)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pe_eint5(self) -> &'a mut W {
        self.variant(PE5_SELECT_A::PE_EINT5)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PE5_SELECT_A::IO_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PE5_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn uart4_rx(self) -> &'a mut W {
        self.variant(PE5_SELECT_A::UART4_RX)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn ledc_do(self) -> &'a mut W {
        self.variant(PE5_SELECT_A::LEDC_DO)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn r_jtag_di(self) -> &'a mut W {
        self.variant(PE5_SELECT_A::R_JTAG_DI)
    }
}
#[doc = "Field `pe6_select` reader - PE6 Select"]
pub type PE6_SELECT_R = crate::FieldReader<u8, PE6_SELECT_A>;
#[doc = "PE6 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PE6_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    NCSI0_D2 = 2,
    #[doc = "4: `100`"]
    TWI3_SCK = 4,
    #[doc = "6: `110`"]
    D_JTAG_DO = 6,
    #[doc = "8: `1000`"]
    RMII_TXCTRL_RMII_TXEN = 8,
    #[doc = "14: `1110`"]
    PE_EINT6 = 14,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    UART5_TX = 3,
    #[doc = "5: `101`"]
    OWA_IN = 5,
    #[doc = "7: `111`"]
    R_JTAG_DO = 7,
}
impl From<PE6_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PE6_SELECT_A) -> Self {
        variant as _
    }
}
impl PE6_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PE6_SELECT_A> {
        match self.bits {
            0 => Some(PE6_SELECT_A::INPUT),
            2 => Some(PE6_SELECT_A::NCSI0_D2),
            4 => Some(PE6_SELECT_A::TWI3_SCK),
            6 => Some(PE6_SELECT_A::D_JTAG_DO),
            8 => Some(PE6_SELECT_A::RMII_TXCTRL_RMII_TXEN),
            14 => Some(PE6_SELECT_A::PE_EINT6),
            15 => Some(PE6_SELECT_A::IO_DISABLE),
            1 => Some(PE6_SELECT_A::OUTPUT),
            3 => Some(PE6_SELECT_A::UART5_TX),
            5 => Some(PE6_SELECT_A::OWA_IN),
            7 => Some(PE6_SELECT_A::R_JTAG_DO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PE6_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `NCSI0_D2`"]
    #[inline(always)]
    pub fn is_ncsi0_d2(&self) -> bool {
        *self == PE6_SELECT_A::NCSI0_D2
    }
    #[doc = "Checks if the value of the field is `TWI3_SCK`"]
    #[inline(always)]
    pub fn is_twi3_sck(&self) -> bool {
        *self == PE6_SELECT_A::TWI3_SCK
    }
    #[doc = "Checks if the value of the field is `D_JTAG_DO`"]
    #[inline(always)]
    pub fn is_d_jtag_do(&self) -> bool {
        *self == PE6_SELECT_A::D_JTAG_DO
    }
    #[doc = "Checks if the value of the field is `RMII_TXCTRL_RMII_TXEN`"]
    #[inline(always)]
    pub fn is_rmii_txctrl_rmii_txen(&self) -> bool {
        *self == PE6_SELECT_A::RMII_TXCTRL_RMII_TXEN
    }
    #[doc = "Checks if the value of the field is `PE_EINT6`"]
    #[inline(always)]
    pub fn is_pe_eint6(&self) -> bool {
        *self == PE6_SELECT_A::PE_EINT6
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PE6_SELECT_A::IO_DISABLE
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PE6_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `UART5_TX`"]
    #[inline(always)]
    pub fn is_uart5_tx(&self) -> bool {
        *self == PE6_SELECT_A::UART5_TX
    }
    #[doc = "Checks if the value of the field is `OWA_IN`"]
    #[inline(always)]
    pub fn is_owa_in(&self) -> bool {
        *self == PE6_SELECT_A::OWA_IN
    }
    #[doc = "Checks if the value of the field is `R_JTAG_DO`"]
    #[inline(always)]
    pub fn is_r_jtag_do(&self) -> bool {
        *self == PE6_SELECT_A::R_JTAG_DO
    }
}
#[doc = "Field `pe6_select` writer - PE6 Select"]
pub type PE6_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PE_CFG0_SPEC, u8, PE6_SELECT_A, 4, O>;
impl<'a, const O: u8> PE6_SELECT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PE6_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn ncsi0_d2(self) -> &'a mut W {
        self.variant(PE6_SELECT_A::NCSI0_D2)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn twi3_sck(self) -> &'a mut W {
        self.variant(PE6_SELECT_A::TWI3_SCK)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn d_jtag_do(self) -> &'a mut W {
        self.variant(PE6_SELECT_A::D_JTAG_DO)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn rmii_txctrl_rmii_txen(self) -> &'a mut W {
        self.variant(PE6_SELECT_A::RMII_TXCTRL_RMII_TXEN)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pe_eint6(self) -> &'a mut W {
        self.variant(PE6_SELECT_A::PE_EINT6)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PE6_SELECT_A::IO_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PE6_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn uart5_tx(self) -> &'a mut W {
        self.variant(PE6_SELECT_A::UART5_TX)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn owa_in(self) -> &'a mut W {
        self.variant(PE6_SELECT_A::OWA_IN)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn r_jtag_do(self) -> &'a mut W {
        self.variant(PE6_SELECT_A::R_JTAG_DO)
    }
}
#[doc = "Field `pe7_select` reader - PE7 Select"]
pub type PE7_SELECT_R = crate::FieldReader<u8, PE7_SELECT_A>;
#[doc = "PE7 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PE7_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    NCSI0_D3 = 2,
    #[doc = "4: `100`"]
    TWI3_SDA = 4,
    #[doc = "6: `110`"]
    D_JTAG_CK = 6,
    #[doc = "8: `1000`"]
    RGMII_CLKIN_RMII_RXER = 8,
    #[doc = "14: `1110`"]
    PE_EINT7 = 14,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    UART5_RX = 3,
    #[doc = "5: `101`"]
    OWA_OUT = 5,
    #[doc = "7: `111`"]
    R_JTAG_CK = 7,
}
impl From<PE7_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PE7_SELECT_A) -> Self {
        variant as _
    }
}
impl PE7_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PE7_SELECT_A> {
        match self.bits {
            0 => Some(PE7_SELECT_A::INPUT),
            2 => Some(PE7_SELECT_A::NCSI0_D3),
            4 => Some(PE7_SELECT_A::TWI3_SDA),
            6 => Some(PE7_SELECT_A::D_JTAG_CK),
            8 => Some(PE7_SELECT_A::RGMII_CLKIN_RMII_RXER),
            14 => Some(PE7_SELECT_A::PE_EINT7),
            15 => Some(PE7_SELECT_A::IO_DISABLE),
            1 => Some(PE7_SELECT_A::OUTPUT),
            3 => Some(PE7_SELECT_A::UART5_RX),
            5 => Some(PE7_SELECT_A::OWA_OUT),
            7 => Some(PE7_SELECT_A::R_JTAG_CK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PE7_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `NCSI0_D3`"]
    #[inline(always)]
    pub fn is_ncsi0_d3(&self) -> bool {
        *self == PE7_SELECT_A::NCSI0_D3
    }
    #[doc = "Checks if the value of the field is `TWI3_SDA`"]
    #[inline(always)]
    pub fn is_twi3_sda(&self) -> bool {
        *self == PE7_SELECT_A::TWI3_SDA
    }
    #[doc = "Checks if the value of the field is `D_JTAG_CK`"]
    #[inline(always)]
    pub fn is_d_jtag_ck(&self) -> bool {
        *self == PE7_SELECT_A::D_JTAG_CK
    }
    #[doc = "Checks if the value of the field is `RGMII_CLKIN_RMII_RXER`"]
    #[inline(always)]
    pub fn is_rgmii_clkin_rmii_rxer(&self) -> bool {
        *self == PE7_SELECT_A::RGMII_CLKIN_RMII_RXER
    }
    #[doc = "Checks if the value of the field is `PE_EINT7`"]
    #[inline(always)]
    pub fn is_pe_eint7(&self) -> bool {
        *self == PE7_SELECT_A::PE_EINT7
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PE7_SELECT_A::IO_DISABLE
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PE7_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `UART5_RX`"]
    #[inline(always)]
    pub fn is_uart5_rx(&self) -> bool {
        *self == PE7_SELECT_A::UART5_RX
    }
    #[doc = "Checks if the value of the field is `OWA_OUT`"]
    #[inline(always)]
    pub fn is_owa_out(&self) -> bool {
        *self == PE7_SELECT_A::OWA_OUT
    }
    #[doc = "Checks if the value of the field is `R_JTAG_CK`"]
    #[inline(always)]
    pub fn is_r_jtag_ck(&self) -> bool {
        *self == PE7_SELECT_A::R_JTAG_CK
    }
}
#[doc = "Field `pe7_select` writer - PE7 Select"]
pub type PE7_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PE_CFG0_SPEC, u8, PE7_SELECT_A, 4, O>;
impl<'a, const O: u8> PE7_SELECT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PE7_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn ncsi0_d3(self) -> &'a mut W {
        self.variant(PE7_SELECT_A::NCSI0_D3)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn twi3_sda(self) -> &'a mut W {
        self.variant(PE7_SELECT_A::TWI3_SDA)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn d_jtag_ck(self) -> &'a mut W {
        self.variant(PE7_SELECT_A::D_JTAG_CK)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn rgmii_clkin_rmii_rxer(self) -> &'a mut W {
        self.variant(PE7_SELECT_A::RGMII_CLKIN_RMII_RXER)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pe_eint7(self) -> &'a mut W {
        self.variant(PE7_SELECT_A::PE_EINT7)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PE7_SELECT_A::IO_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PE7_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn uart5_rx(self) -> &'a mut W {
        self.variant(PE7_SELECT_A::UART5_RX)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn owa_out(self) -> &'a mut W {
        self.variant(PE7_SELECT_A::OWA_OUT)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn r_jtag_ck(self) -> &'a mut W {
        self.variant(PE7_SELECT_A::R_JTAG_CK)
    }
}
impl R {
    #[doc = "Bits 0:3 - PE0 Select"]
    #[inline(always)]
    pub fn pe0_select(&self) -> PE0_SELECT_R {
        PE0_SELECT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PE1 Select"]
    #[inline(always)]
    pub fn pe1_select(&self) -> PE1_SELECT_R {
        PE1_SELECT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PE2 Select"]
    #[inline(always)]
    pub fn pe2_select(&self) -> PE2_SELECT_R {
        PE2_SELECT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PE3 Select"]
    #[inline(always)]
    pub fn pe3_select(&self) -> PE3_SELECT_R {
        PE3_SELECT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PE4 Select"]
    #[inline(always)]
    pub fn pe4_select(&self) -> PE4_SELECT_R {
        PE4_SELECT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PE5 Select"]
    #[inline(always)]
    pub fn pe5_select(&self) -> PE5_SELECT_R {
        PE5_SELECT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PE6 Select"]
    #[inline(always)]
    pub fn pe6_select(&self) -> PE6_SELECT_R {
        PE6_SELECT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - PE7 Select"]
    #[inline(always)]
    pub fn pe7_select(&self) -> PE7_SELECT_R {
        PE7_SELECT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PE0 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pe0_select(&mut self) -> PE0_SELECT_W<0> {
        PE0_SELECT_W::new(self)
    }
    #[doc = "Bits 4:7 - PE1 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pe1_select(&mut self) -> PE1_SELECT_W<4> {
        PE1_SELECT_W::new(self)
    }
    #[doc = "Bits 8:11 - PE2 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pe2_select(&mut self) -> PE2_SELECT_W<8> {
        PE2_SELECT_W::new(self)
    }
    #[doc = "Bits 12:15 - PE3 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pe3_select(&mut self) -> PE3_SELECT_W<12> {
        PE3_SELECT_W::new(self)
    }
    #[doc = "Bits 16:19 - PE4 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pe4_select(&mut self) -> PE4_SELECT_W<16> {
        PE4_SELECT_W::new(self)
    }
    #[doc = "Bits 20:23 - PE5 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pe5_select(&mut self) -> PE5_SELECT_W<20> {
        PE5_SELECT_W::new(self)
    }
    #[doc = "Bits 24:27 - PE6 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pe6_select(&mut self) -> PE6_SELECT_W<24> {
        PE6_SELECT_W::new(self)
    }
    #[doc = "Bits 28:31 - PE7 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pe7_select(&mut self) -> PE7_SELECT_W<28> {
        PE7_SELECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PE Configure Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pe_cfg0](index.html) module"]
pub struct PE_CFG0_SPEC;
impl crate::RegisterSpec for PE_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pe_cfg0::R](R) reader structure"]
impl crate::Readable for PE_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pe_cfg0::W](W) writer structure"]
impl crate::Writable for PE_CFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pe_cfg0 to value 0"]
impl crate::Resettable for PE_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
