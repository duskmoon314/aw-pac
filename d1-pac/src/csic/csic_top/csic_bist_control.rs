#[doc = "Register `csic_bist_control` reader"]
pub type R = crate::R<CSIC_BIST_CONTROL_SPEC>;
#[doc = "Register `csic_bist_control` writer"]
pub type W = crate::W<CSIC_BIST_CONTROL_SPEC>;
#[doc = "Field `bist_en` reader - BIST enable\n\nA positive will trigger the BIST to start."]
pub type BIST_EN_R = crate::BitReader;
#[doc = "Field `bist_en` writer - BIST enable\n\nA positive will trigger the BIST to start."]
pub type BIST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bist_wdata_pat` reader - BIST write data pattern"]
pub type BIST_WDATA_PAT_R = crate::FieldReader<BIST_WDATA_PAT_A>;
#[doc = "BIST write data pattern\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BIST_WDATA_PAT_A {
    #[doc = "0: `0`"]
    _0X00000000 = 0,
    #[doc = "1: `1`"]
    _0X55555555 = 1,
    #[doc = "2: `10`"]
    _0X33333333 = 2,
    #[doc = "3: `11`"]
    _0X0F0F0F0F = 3,
    #[doc = "4: `100`"]
    _0X00FF00FF = 4,
    #[doc = "5: `101`"]
    _0X0000FFFF = 5,
}
impl From<BIST_WDATA_PAT_A> for u8 {
    #[inline(always)]
    fn from(variant: BIST_WDATA_PAT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BIST_WDATA_PAT_A {
    type Ux = u8;
}
impl BIST_WDATA_PAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BIST_WDATA_PAT_A> {
        match self.bits {
            0 => Some(BIST_WDATA_PAT_A::_0X00000000),
            1 => Some(BIST_WDATA_PAT_A::_0X55555555),
            2 => Some(BIST_WDATA_PAT_A::_0X33333333),
            3 => Some(BIST_WDATA_PAT_A::_0X0F0F0F0F),
            4 => Some(BIST_WDATA_PAT_A::_0X00FF00FF),
            5 => Some(BIST_WDATA_PAT_A::_0X0000FFFF),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_0x00000000(&self) -> bool {
        *self == BIST_WDATA_PAT_A::_0X00000000
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_0x55555555(&self) -> bool {
        *self == BIST_WDATA_PAT_A::_0X55555555
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_0x33333333(&self) -> bool {
        *self == BIST_WDATA_PAT_A::_0X33333333
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_0x0f0f0f0f(&self) -> bool {
        *self == BIST_WDATA_PAT_A::_0X0F0F0F0F
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_0x00ff00ff(&self) -> bool {
        *self == BIST_WDATA_PAT_A::_0X00FF00FF
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_0x0000ffff(&self) -> bool {
        *self == BIST_WDATA_PAT_A::_0X0000FFFF
    }
}
#[doc = "Field `bist_wdata_pat` writer - BIST write data pattern"]
pub type BIST_WDATA_PAT_W<'a, REG> = crate::FieldWriter<'a, REG, 3, BIST_WDATA_PAT_A>;
impl<'a, REG> BIST_WDATA_PAT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _0x00000000(self) -> &'a mut crate::W<REG> {
        self.variant(BIST_WDATA_PAT_A::_0X00000000)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _0x55555555(self) -> &'a mut crate::W<REG> {
        self.variant(BIST_WDATA_PAT_A::_0X55555555)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _0x33333333(self) -> &'a mut crate::W<REG> {
        self.variant(BIST_WDATA_PAT_A::_0X33333333)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn _0x0f0f0f0f(self) -> &'a mut crate::W<REG> {
        self.variant(BIST_WDATA_PAT_A::_0X0F0F0F0F)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn _0x00ff00ff(self) -> &'a mut crate::W<REG> {
        self.variant(BIST_WDATA_PAT_A::_0X00FF00FF)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn _0x0000ffff(self) -> &'a mut crate::W<REG> {
        self.variant(BIST_WDATA_PAT_A::_0X0000FFFF)
    }
}
#[doc = "Field `bist_addr_mode_sel` reader - BIST address mode select"]
pub type BIST_ADDR_MODE_SEL_R = crate::BitReader;
#[doc = "Field `bist_addr_mode_sel` writer - BIST address mode select"]
pub type BIST_ADDR_MODE_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bist_reg_sel` reader - BIST register select"]
pub type BIST_REG_SEL_R = crate::FieldReader;
#[doc = "Field `bist_reg_sel` writer - BIST register select"]
pub type BIST_REG_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `bist_busy` reader - BIST busy"]
pub type BIST_BUSY_R = crate::BitReader<BIST_BUSY_A>;
#[doc = "BIST busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIST_BUSY_A {
    #[doc = "0: `0`"]
    IDLE = 0,
    #[doc = "1: `1`"]
    BUSY = 1,
}
impl From<BIST_BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BIST_BUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl BIST_BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BIST_BUSY_A {
        match self.bits {
            false => BIST_BUSY_A::IDLE,
            true => BIST_BUSY_A::BUSY,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == BIST_BUSY_A::IDLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BIST_BUSY_A::BUSY
    }
}
#[doc = "Field `bist_stop` reader - BIST stop"]
pub type BIST_STOP_R = crate::BitReader<BIST_STOP_A>;
#[doc = "BIST stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIST_STOP_A {
    #[doc = "0: `0`"]
    RUNNING = 0,
    #[doc = "1: `1`"]
    STOP = 1,
}
impl From<BIST_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: BIST_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl BIST_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BIST_STOP_A {
        match self.bits {
            false => BIST_STOP_A::RUNNING,
            true => BIST_STOP_A::STOP,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == BIST_STOP_A::RUNNING
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == BIST_STOP_A::STOP
    }
}
#[doc = "Field `bist_err_cyc` reader - BIST error cycle"]
pub type BIST_ERR_CYC_R = crate::FieldReader;
#[doc = "Field `bist_err_pat` reader - BIST error pattern"]
pub type BIST_ERR_PAT_R = crate::FieldReader;
#[doc = "Field `bist_err_sta` reader - BIST error status"]
pub type BIST_ERR_STA_R = crate::BitReader<BIST_ERR_STA_A>;
#[doc = "BIST error status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIST_ERR_STA_A {
    #[doc = "0: `0`"]
    NO_EFFECT = 0,
    #[doc = "1: `1`"]
    ERROR = 1,
}
impl From<BIST_ERR_STA_A> for bool {
    #[inline(always)]
    fn from(variant: BIST_ERR_STA_A) -> Self {
        variant as u8 != 0
    }
}
impl BIST_ERR_STA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BIST_ERR_STA_A {
        match self.bits {
            false => BIST_ERR_STA_A::NO_EFFECT,
            true => BIST_ERR_STA_A::ERROR,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == BIST_ERR_STA_A::NO_EFFECT
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == BIST_ERR_STA_A::ERROR
    }
}
impl R {
    #[doc = "Bit 0 - BIST enable\n\nA positive will trigger the BIST to start."]
    #[inline(always)]
    pub fn bist_en(&self) -> BIST_EN_R {
        BIST_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - BIST write data pattern"]
    #[inline(always)]
    pub fn bist_wdata_pat(&self) -> BIST_WDATA_PAT_R {
        BIST_WDATA_PAT_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - BIST address mode select"]
    #[inline(always)]
    pub fn bist_addr_mode_sel(&self) -> BIST_ADDR_MODE_SEL_R {
        BIST_ADDR_MODE_SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - BIST register select"]
    #[inline(always)]
    pub fn bist_reg_sel(&self) -> BIST_REG_SEL_R {
        BIST_REG_SEL_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - BIST busy"]
    #[inline(always)]
    pub fn bist_busy(&self) -> BIST_BUSY_R {
        BIST_BUSY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BIST stop"]
    #[inline(always)]
    pub fn bist_stop(&self) -> BIST_STOP_R {
        BIST_STOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - BIST error cycle"]
    #[inline(always)]
    pub fn bist_err_cyc(&self) -> BIST_ERR_CYC_R {
        BIST_ERR_CYC_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:14 - BIST error pattern"]
    #[inline(always)]
    pub fn bist_err_pat(&self) -> BIST_ERR_PAT_R {
        BIST_ERR_PAT_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - BIST error status"]
    #[inline(always)]
    pub fn bist_err_sta(&self) -> BIST_ERR_STA_R {
        BIST_ERR_STA_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BIST enable\n\nA positive will trigger the BIST to start."]
    #[inline(always)]
    #[must_use]
    pub fn bist_en(&mut self) -> BIST_EN_W<CSIC_BIST_CONTROL_SPEC> {
        BIST_EN_W::new(self, 0)
    }
    #[doc = "Bits 1:3 - BIST write data pattern"]
    #[inline(always)]
    #[must_use]
    pub fn bist_wdata_pat(&mut self) -> BIST_WDATA_PAT_W<CSIC_BIST_CONTROL_SPEC> {
        BIST_WDATA_PAT_W::new(self, 1)
    }
    #[doc = "Bit 4 - BIST address mode select"]
    #[inline(always)]
    #[must_use]
    pub fn bist_addr_mode_sel(&mut self) -> BIST_ADDR_MODE_SEL_W<CSIC_BIST_CONTROL_SPEC> {
        BIST_ADDR_MODE_SEL_W::new(self, 4)
    }
    #[doc = "Bits 5:7 - BIST register select"]
    #[inline(always)]
    #[must_use]
    pub fn bist_reg_sel(&mut self) -> BIST_REG_SEL_W<CSIC_BIST_CONTROL_SPEC> {
        BIST_REG_SEL_W::new(self, 5)
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
#[doc = "CSIC BIST Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_bist_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_bist_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSIC_BIST_CONTROL_SPEC;
impl crate::RegisterSpec for CSIC_BIST_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csic_bist_control::R`](R) reader structure"]
impl crate::Readable for CSIC_BIST_CONTROL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csic_bist_control::W`](W) writer structure"]
impl crate::Writable for CSIC_BIST_CONTROL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_bist_control to value 0x0200"]
impl crate::Resettable for CSIC_BIST_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
