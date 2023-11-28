#[doc = "Register `pd_cfg0` reader"]
pub type R = crate::R<PD_CFG0_SPEC>;
#[doc = "Register `pd_cfg0` writer"]
pub type W = crate::W<PD_CFG0_SPEC>;
#[doc = "Field `pd0_select` reader - PD0 Select"]
pub type PD0_SELECT_R = crate::FieldReader<PD0_SELECT_A>;
#[doc = "PD0 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PD0_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    LCD0_D2 = 2,
    #[doc = "4: `100`"]
    DSI_D0P = 4,
    #[doc = "14: `1110`"]
    PD_EINT0 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    LVDS0_V0P = 3,
    #[doc = "5: `101`"]
    TWI0_SCK = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PD0_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PD0_SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PD0_SELECT_A {
    type Ux = u8;
}
impl PD0_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PD0_SELECT_A> {
        match self.bits {
            0 => Some(PD0_SELECT_A::INPUT),
            2 => Some(PD0_SELECT_A::LCD0_D2),
            4 => Some(PD0_SELECT_A::DSI_D0P),
            14 => Some(PD0_SELECT_A::PD_EINT0),
            1 => Some(PD0_SELECT_A::OUTPUT),
            3 => Some(PD0_SELECT_A::LVDS0_V0P),
            5 => Some(PD0_SELECT_A::TWI0_SCK),
            15 => Some(PD0_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PD0_SELECT_A::INPUT
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_lcd0_d2(&self) -> bool {
        *self == PD0_SELECT_A::LCD0_D2
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_dsi_d0p(&self) -> bool {
        *self == PD0_SELECT_A::DSI_D0P
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_pd_eint0(&self) -> bool {
        *self == PD0_SELECT_A::PD_EINT0
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PD0_SELECT_A::OUTPUT
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_lvds0_v0p(&self) -> bool {
        *self == PD0_SELECT_A::LVDS0_V0P
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_twi0_sck(&self) -> bool {
        *self == PD0_SELECT_A::TWI0_SCK
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PD0_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pd0_select` writer - PD0 Select"]
pub type PD0_SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PD0_SELECT_A>;
impl<'a, REG> PD0_SELECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(PD0_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn lcd0_d2(self) -> &'a mut crate::W<REG> {
        self.variant(PD0_SELECT_A::LCD0_D2)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn dsi_d0p(self) -> &'a mut crate::W<REG> {
        self.variant(PD0_SELECT_A::DSI_D0P)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pd_eint0(self) -> &'a mut crate::W<REG> {
        self.variant(PD0_SELECT_A::PD_EINT0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(PD0_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn lvds0_v0p(self) -> &'a mut crate::W<REG> {
        self.variant(PD0_SELECT_A::LVDS0_V0P)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn twi0_sck(self) -> &'a mut crate::W<REG> {
        self.variant(PD0_SELECT_A::TWI0_SCK)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PD0_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pd1_select` reader - PD1 Select"]
pub type PD1_SELECT_R = crate::FieldReader<PD1_SELECT_A>;
#[doc = "PD1 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PD1_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    LCD0_D3 = 2,
    #[doc = "4: `100`"]
    DSI_D0N = 4,
    #[doc = "14: `1110`"]
    PD_EINT1 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    LVDS0_V0N = 3,
    #[doc = "5: `101`"]
    UART2_TX = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PD1_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PD1_SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PD1_SELECT_A {
    type Ux = u8;
}
impl PD1_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PD1_SELECT_A> {
        match self.bits {
            0 => Some(PD1_SELECT_A::INPUT),
            2 => Some(PD1_SELECT_A::LCD0_D3),
            4 => Some(PD1_SELECT_A::DSI_D0N),
            14 => Some(PD1_SELECT_A::PD_EINT1),
            1 => Some(PD1_SELECT_A::OUTPUT),
            3 => Some(PD1_SELECT_A::LVDS0_V0N),
            5 => Some(PD1_SELECT_A::UART2_TX),
            15 => Some(PD1_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PD1_SELECT_A::INPUT
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_lcd0_d3(&self) -> bool {
        *self == PD1_SELECT_A::LCD0_D3
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_dsi_d0n(&self) -> bool {
        *self == PD1_SELECT_A::DSI_D0N
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_pd_eint1(&self) -> bool {
        *self == PD1_SELECT_A::PD_EINT1
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PD1_SELECT_A::OUTPUT
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_lvds0_v0n(&self) -> bool {
        *self == PD1_SELECT_A::LVDS0_V0N
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_uart2_tx(&self) -> bool {
        *self == PD1_SELECT_A::UART2_TX
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PD1_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pd1_select` writer - PD1 Select"]
pub type PD1_SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PD1_SELECT_A>;
impl<'a, REG> PD1_SELECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(PD1_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn lcd0_d3(self) -> &'a mut crate::W<REG> {
        self.variant(PD1_SELECT_A::LCD0_D3)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn dsi_d0n(self) -> &'a mut crate::W<REG> {
        self.variant(PD1_SELECT_A::DSI_D0N)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pd_eint1(self) -> &'a mut crate::W<REG> {
        self.variant(PD1_SELECT_A::PD_EINT1)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(PD1_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn lvds0_v0n(self) -> &'a mut crate::W<REG> {
        self.variant(PD1_SELECT_A::LVDS0_V0N)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn uart2_tx(self) -> &'a mut crate::W<REG> {
        self.variant(PD1_SELECT_A::UART2_TX)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PD1_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pd2_select` reader - PD2 Select"]
pub type PD2_SELECT_R = crate::FieldReader<PD2_SELECT_A>;
#[doc = "PD2 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PD2_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    LCD0_D4 = 2,
    #[doc = "4: `100`"]
    DSI_D1P = 4,
    #[doc = "14: `1110`"]
    PD_EINT2 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    LVDS0_V1P = 3,
    #[doc = "5: `101`"]
    UART2_RX = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PD2_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PD2_SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PD2_SELECT_A {
    type Ux = u8;
}
impl PD2_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PD2_SELECT_A> {
        match self.bits {
            0 => Some(PD2_SELECT_A::INPUT),
            2 => Some(PD2_SELECT_A::LCD0_D4),
            4 => Some(PD2_SELECT_A::DSI_D1P),
            14 => Some(PD2_SELECT_A::PD_EINT2),
            1 => Some(PD2_SELECT_A::OUTPUT),
            3 => Some(PD2_SELECT_A::LVDS0_V1P),
            5 => Some(PD2_SELECT_A::UART2_RX),
            15 => Some(PD2_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PD2_SELECT_A::INPUT
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_lcd0_d4(&self) -> bool {
        *self == PD2_SELECT_A::LCD0_D4
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_dsi_d1p(&self) -> bool {
        *self == PD2_SELECT_A::DSI_D1P
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_pd_eint2(&self) -> bool {
        *self == PD2_SELECT_A::PD_EINT2
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PD2_SELECT_A::OUTPUT
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_lvds0_v1p(&self) -> bool {
        *self == PD2_SELECT_A::LVDS0_V1P
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_uart2_rx(&self) -> bool {
        *self == PD2_SELECT_A::UART2_RX
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PD2_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pd2_select` writer - PD2 Select"]
pub type PD2_SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PD2_SELECT_A>;
impl<'a, REG> PD2_SELECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(PD2_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn lcd0_d4(self) -> &'a mut crate::W<REG> {
        self.variant(PD2_SELECT_A::LCD0_D4)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn dsi_d1p(self) -> &'a mut crate::W<REG> {
        self.variant(PD2_SELECT_A::DSI_D1P)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pd_eint2(self) -> &'a mut crate::W<REG> {
        self.variant(PD2_SELECT_A::PD_EINT2)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(PD2_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn lvds0_v1p(self) -> &'a mut crate::W<REG> {
        self.variant(PD2_SELECT_A::LVDS0_V1P)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn uart2_rx(self) -> &'a mut crate::W<REG> {
        self.variant(PD2_SELECT_A::UART2_RX)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PD2_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pd3_select` reader - PD3 Select"]
pub type PD3_SELECT_R = crate::FieldReader<PD3_SELECT_A>;
#[doc = "PD3 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PD3_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    LCD0_D5 = 2,
    #[doc = "4: `100`"]
    DSI_D1N = 4,
    #[doc = "14: `1110`"]
    PD_EINT3 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    LVDS0_V1N = 3,
    #[doc = "5: `101`"]
    UART2_RTS = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PD3_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PD3_SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PD3_SELECT_A {
    type Ux = u8;
}
impl PD3_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PD3_SELECT_A> {
        match self.bits {
            0 => Some(PD3_SELECT_A::INPUT),
            2 => Some(PD3_SELECT_A::LCD0_D5),
            4 => Some(PD3_SELECT_A::DSI_D1N),
            14 => Some(PD3_SELECT_A::PD_EINT3),
            1 => Some(PD3_SELECT_A::OUTPUT),
            3 => Some(PD3_SELECT_A::LVDS0_V1N),
            5 => Some(PD3_SELECT_A::UART2_RTS),
            15 => Some(PD3_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PD3_SELECT_A::INPUT
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_lcd0_d5(&self) -> bool {
        *self == PD3_SELECT_A::LCD0_D5
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_dsi_d1n(&self) -> bool {
        *self == PD3_SELECT_A::DSI_D1N
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_pd_eint3(&self) -> bool {
        *self == PD3_SELECT_A::PD_EINT3
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PD3_SELECT_A::OUTPUT
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_lvds0_v1n(&self) -> bool {
        *self == PD3_SELECT_A::LVDS0_V1N
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_uart2_rts(&self) -> bool {
        *self == PD3_SELECT_A::UART2_RTS
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PD3_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pd3_select` writer - PD3 Select"]
pub type PD3_SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PD3_SELECT_A>;
impl<'a, REG> PD3_SELECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(PD3_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn lcd0_d5(self) -> &'a mut crate::W<REG> {
        self.variant(PD3_SELECT_A::LCD0_D5)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn dsi_d1n(self) -> &'a mut crate::W<REG> {
        self.variant(PD3_SELECT_A::DSI_D1N)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pd_eint3(self) -> &'a mut crate::W<REG> {
        self.variant(PD3_SELECT_A::PD_EINT3)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(PD3_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn lvds0_v1n(self) -> &'a mut crate::W<REG> {
        self.variant(PD3_SELECT_A::LVDS0_V1N)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn uart2_rts(self) -> &'a mut crate::W<REG> {
        self.variant(PD3_SELECT_A::UART2_RTS)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PD3_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pd4_select` reader - PD4 Select"]
pub type PD4_SELECT_R = crate::FieldReader<PD4_SELECT_A>;
#[doc = "PD4 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PD4_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    LCD0_D6 = 2,
    #[doc = "4: `100`"]
    DSI_CKP = 4,
    #[doc = "14: `1110`"]
    PD_EINT4 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    LVDS0_V2P = 3,
    #[doc = "5: `101`"]
    UART2_CTS = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PD4_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PD4_SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PD4_SELECT_A {
    type Ux = u8;
}
impl PD4_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PD4_SELECT_A> {
        match self.bits {
            0 => Some(PD4_SELECT_A::INPUT),
            2 => Some(PD4_SELECT_A::LCD0_D6),
            4 => Some(PD4_SELECT_A::DSI_CKP),
            14 => Some(PD4_SELECT_A::PD_EINT4),
            1 => Some(PD4_SELECT_A::OUTPUT),
            3 => Some(PD4_SELECT_A::LVDS0_V2P),
            5 => Some(PD4_SELECT_A::UART2_CTS),
            15 => Some(PD4_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PD4_SELECT_A::INPUT
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_lcd0_d6(&self) -> bool {
        *self == PD4_SELECT_A::LCD0_D6
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_dsi_ckp(&self) -> bool {
        *self == PD4_SELECT_A::DSI_CKP
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_pd_eint4(&self) -> bool {
        *self == PD4_SELECT_A::PD_EINT4
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PD4_SELECT_A::OUTPUT
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_lvds0_v2p(&self) -> bool {
        *self == PD4_SELECT_A::LVDS0_V2P
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_uart2_cts(&self) -> bool {
        *self == PD4_SELECT_A::UART2_CTS
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PD4_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pd4_select` writer - PD4 Select"]
pub type PD4_SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PD4_SELECT_A>;
impl<'a, REG> PD4_SELECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(PD4_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn lcd0_d6(self) -> &'a mut crate::W<REG> {
        self.variant(PD4_SELECT_A::LCD0_D6)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn dsi_ckp(self) -> &'a mut crate::W<REG> {
        self.variant(PD4_SELECT_A::DSI_CKP)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pd_eint4(self) -> &'a mut crate::W<REG> {
        self.variant(PD4_SELECT_A::PD_EINT4)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(PD4_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn lvds0_v2p(self) -> &'a mut crate::W<REG> {
        self.variant(PD4_SELECT_A::LVDS0_V2P)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn uart2_cts(self) -> &'a mut crate::W<REG> {
        self.variant(PD4_SELECT_A::UART2_CTS)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PD4_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pd5_select` reader - PD5 Select"]
pub type PD5_SELECT_R = crate::FieldReader<PD5_SELECT_A>;
#[doc = "PD5 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PD5_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    LCD0_D7 = 2,
    #[doc = "4: `100`"]
    DSI_CKN = 4,
    #[doc = "14: `1110`"]
    PD_EINT5 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    LVDS0_V2N = 3,
    #[doc = "5: `101`"]
    UART5_TX = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PD5_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PD5_SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PD5_SELECT_A {
    type Ux = u8;
}
impl PD5_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PD5_SELECT_A> {
        match self.bits {
            0 => Some(PD5_SELECT_A::INPUT),
            2 => Some(PD5_SELECT_A::LCD0_D7),
            4 => Some(PD5_SELECT_A::DSI_CKN),
            14 => Some(PD5_SELECT_A::PD_EINT5),
            1 => Some(PD5_SELECT_A::OUTPUT),
            3 => Some(PD5_SELECT_A::LVDS0_V2N),
            5 => Some(PD5_SELECT_A::UART5_TX),
            15 => Some(PD5_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PD5_SELECT_A::INPUT
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_lcd0_d7(&self) -> bool {
        *self == PD5_SELECT_A::LCD0_D7
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_dsi_ckn(&self) -> bool {
        *self == PD5_SELECT_A::DSI_CKN
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_pd_eint5(&self) -> bool {
        *self == PD5_SELECT_A::PD_EINT5
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PD5_SELECT_A::OUTPUT
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_lvds0_v2n(&self) -> bool {
        *self == PD5_SELECT_A::LVDS0_V2N
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_uart5_tx(&self) -> bool {
        *self == PD5_SELECT_A::UART5_TX
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PD5_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pd5_select` writer - PD5 Select"]
pub type PD5_SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PD5_SELECT_A>;
impl<'a, REG> PD5_SELECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(PD5_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn lcd0_d7(self) -> &'a mut crate::W<REG> {
        self.variant(PD5_SELECT_A::LCD0_D7)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn dsi_ckn(self) -> &'a mut crate::W<REG> {
        self.variant(PD5_SELECT_A::DSI_CKN)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pd_eint5(self) -> &'a mut crate::W<REG> {
        self.variant(PD5_SELECT_A::PD_EINT5)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(PD5_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn lvds0_v2n(self) -> &'a mut crate::W<REG> {
        self.variant(PD5_SELECT_A::LVDS0_V2N)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn uart5_tx(self) -> &'a mut crate::W<REG> {
        self.variant(PD5_SELECT_A::UART5_TX)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PD5_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pd6_select` reader - PD6 Select"]
pub type PD6_SELECT_R = crate::FieldReader<PD6_SELECT_A>;
#[doc = "PD6 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PD6_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    LCD0_D10 = 2,
    #[doc = "4: `100`"]
    DSI_D2P = 4,
    #[doc = "14: `1110`"]
    PD_EINT6 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    LVDS0_CKP = 3,
    #[doc = "5: `101`"]
    UART5_RX = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PD6_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PD6_SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PD6_SELECT_A {
    type Ux = u8;
}
impl PD6_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PD6_SELECT_A> {
        match self.bits {
            0 => Some(PD6_SELECT_A::INPUT),
            2 => Some(PD6_SELECT_A::LCD0_D10),
            4 => Some(PD6_SELECT_A::DSI_D2P),
            14 => Some(PD6_SELECT_A::PD_EINT6),
            1 => Some(PD6_SELECT_A::OUTPUT),
            3 => Some(PD6_SELECT_A::LVDS0_CKP),
            5 => Some(PD6_SELECT_A::UART5_RX),
            15 => Some(PD6_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PD6_SELECT_A::INPUT
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_lcd0_d10(&self) -> bool {
        *self == PD6_SELECT_A::LCD0_D10
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_dsi_d2p(&self) -> bool {
        *self == PD6_SELECT_A::DSI_D2P
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_pd_eint6(&self) -> bool {
        *self == PD6_SELECT_A::PD_EINT6
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PD6_SELECT_A::OUTPUT
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_lvds0_ckp(&self) -> bool {
        *self == PD6_SELECT_A::LVDS0_CKP
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_uart5_rx(&self) -> bool {
        *self == PD6_SELECT_A::UART5_RX
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PD6_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pd6_select` writer - PD6 Select"]
pub type PD6_SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PD6_SELECT_A>;
impl<'a, REG> PD6_SELECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(PD6_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn lcd0_d10(self) -> &'a mut crate::W<REG> {
        self.variant(PD6_SELECT_A::LCD0_D10)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn dsi_d2p(self) -> &'a mut crate::W<REG> {
        self.variant(PD6_SELECT_A::DSI_D2P)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pd_eint6(self) -> &'a mut crate::W<REG> {
        self.variant(PD6_SELECT_A::PD_EINT6)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(PD6_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn lvds0_ckp(self) -> &'a mut crate::W<REG> {
        self.variant(PD6_SELECT_A::LVDS0_CKP)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn uart5_rx(self) -> &'a mut crate::W<REG> {
        self.variant(PD6_SELECT_A::UART5_RX)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PD6_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pd7_select` reader - PD7 Select"]
pub type PD7_SELECT_R = crate::FieldReader<PD7_SELECT_A>;
#[doc = "PD7 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PD7_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    LCD0_D11 = 2,
    #[doc = "4: `100`"]
    DSI_D2N = 4,
    #[doc = "14: `1110`"]
    PD_EINT7 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    LVDS0_CKN = 3,
    #[doc = "5: `101`"]
    UART4_TX = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PD7_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PD7_SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PD7_SELECT_A {
    type Ux = u8;
}
impl PD7_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PD7_SELECT_A> {
        match self.bits {
            0 => Some(PD7_SELECT_A::INPUT),
            2 => Some(PD7_SELECT_A::LCD0_D11),
            4 => Some(PD7_SELECT_A::DSI_D2N),
            14 => Some(PD7_SELECT_A::PD_EINT7),
            1 => Some(PD7_SELECT_A::OUTPUT),
            3 => Some(PD7_SELECT_A::LVDS0_CKN),
            5 => Some(PD7_SELECT_A::UART4_TX),
            15 => Some(PD7_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PD7_SELECT_A::INPUT
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_lcd0_d11(&self) -> bool {
        *self == PD7_SELECT_A::LCD0_D11
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_dsi_d2n(&self) -> bool {
        *self == PD7_SELECT_A::DSI_D2N
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_pd_eint7(&self) -> bool {
        *self == PD7_SELECT_A::PD_EINT7
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PD7_SELECT_A::OUTPUT
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_lvds0_ckn(&self) -> bool {
        *self == PD7_SELECT_A::LVDS0_CKN
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_uart4_tx(&self) -> bool {
        *self == PD7_SELECT_A::UART4_TX
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PD7_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pd7_select` writer - PD7 Select"]
pub type PD7_SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PD7_SELECT_A>;
impl<'a, REG> PD7_SELECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(PD7_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn lcd0_d11(self) -> &'a mut crate::W<REG> {
        self.variant(PD7_SELECT_A::LCD0_D11)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn dsi_d2n(self) -> &'a mut crate::W<REG> {
        self.variant(PD7_SELECT_A::DSI_D2N)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pd_eint7(self) -> &'a mut crate::W<REG> {
        self.variant(PD7_SELECT_A::PD_EINT7)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(PD7_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn lvds0_ckn(self) -> &'a mut crate::W<REG> {
        self.variant(PD7_SELECT_A::LVDS0_CKN)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn uart4_tx(self) -> &'a mut crate::W<REG> {
        self.variant(PD7_SELECT_A::UART4_TX)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PD7_SELECT_A::IO_DISABLE)
    }
}
impl R {
    #[doc = "Bits 0:3 - PD0 Select"]
    #[inline(always)]
    pub fn pd0_select(&self) -> PD0_SELECT_R {
        PD0_SELECT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PD1 Select"]
    #[inline(always)]
    pub fn pd1_select(&self) -> PD1_SELECT_R {
        PD1_SELECT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PD2 Select"]
    #[inline(always)]
    pub fn pd2_select(&self) -> PD2_SELECT_R {
        PD2_SELECT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PD3 Select"]
    #[inline(always)]
    pub fn pd3_select(&self) -> PD3_SELECT_R {
        PD3_SELECT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PD4 Select"]
    #[inline(always)]
    pub fn pd4_select(&self) -> PD4_SELECT_R {
        PD4_SELECT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PD5 Select"]
    #[inline(always)]
    pub fn pd5_select(&self) -> PD5_SELECT_R {
        PD5_SELECT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PD6 Select"]
    #[inline(always)]
    pub fn pd6_select(&self) -> PD6_SELECT_R {
        PD6_SELECT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - PD7 Select"]
    #[inline(always)]
    pub fn pd7_select(&self) -> PD7_SELECT_R {
        PD7_SELECT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PD0 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pd0_select(&mut self) -> PD0_SELECT_W<PD_CFG0_SPEC> {
        PD0_SELECT_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - PD1 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pd1_select(&mut self) -> PD1_SELECT_W<PD_CFG0_SPEC> {
        PD1_SELECT_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - PD2 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pd2_select(&mut self) -> PD2_SELECT_W<PD_CFG0_SPEC> {
        PD2_SELECT_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - PD3 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pd3_select(&mut self) -> PD3_SELECT_W<PD_CFG0_SPEC> {
        PD3_SELECT_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - PD4 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pd4_select(&mut self) -> PD4_SELECT_W<PD_CFG0_SPEC> {
        PD4_SELECT_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - PD5 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pd5_select(&mut self) -> PD5_SELECT_W<PD_CFG0_SPEC> {
        PD5_SELECT_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - PD6 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pd6_select(&mut self) -> PD6_SELECT_W<PD_CFG0_SPEC> {
        PD6_SELECT_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - PD7 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pd7_select(&mut self) -> PD7_SELECT_W<PD_CFG0_SPEC> {
        PD7_SELECT_W::new(self, 28)
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
#[doc = "PD Configure Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd_cfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_cfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PD_CFG0_SPEC;
impl crate::RegisterSpec for PD_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pd_cfg0::R`](R) reader structure"]
impl crate::Readable for PD_CFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pd_cfg0::W`](W) writer structure"]
impl crate::Writable for PD_CFG0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pd_cfg0 to value 0"]
impl crate::Resettable for PD_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
