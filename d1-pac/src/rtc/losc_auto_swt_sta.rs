#[doc = "Register `losc_auto_swt_sta` reader"]
pub type R = crate::R<LOSC_AUTO_SWT_STA_SPEC>;
#[doc = "Register `losc_auto_swt_sta` writer"]
pub type W = crate::W<LOSC_AUTO_SWT_STA_SPEC>;
#[doc = "Field `losc_src_sel_sta` reader - Checking LOSC clock source status"]
pub type LOSC_SRC_SEL_STA_R = crate::BitReader<LOSC_SRC_SEL_STA_A>;
#[doc = "Checking LOSC clock source status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOSC_SRC_SEL_STA_A {
    #[doc = "0: Low frequency clock from 16M RC"]
    LOW = 0,
    #[doc = "1: External 32.768 kHz OSC"]
    EXTERNAL = 1,
}
impl From<LOSC_SRC_SEL_STA_A> for bool {
    #[inline(always)]
    fn from(variant: LOSC_SRC_SEL_STA_A) -> Self {
        variant as u8 != 0
    }
}
impl LOSC_SRC_SEL_STA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LOSC_SRC_SEL_STA_A {
        match self.bits {
            false => LOSC_SRC_SEL_STA_A::LOW,
            true => LOSC_SRC_SEL_STA_A::EXTERNAL,
        }
    }
    #[doc = "Low frequency clock from 16M RC"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == LOSC_SRC_SEL_STA_A::LOW
    }
    #[doc = "External 32.768 kHz OSC"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == LOSC_SRC_SEL_STA_A::EXTERNAL
    }
}
#[doc = "Field `losc_auto_swt_pend` reader - LOSC auto switch pending"]
pub type LOSC_AUTO_SWT_PEND_R = crate::BitReader<LOSC_AUTO_SWT_PEND_A>;
#[doc = "LOSC auto switch pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOSC_AUTO_SWT_PEND_A {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Auto switch pending, it means LOSC_SRC_SEL is changed from 1 to 0.\n\nSetting 1 to this bit will clear it."]
    AUTO = 1,
}
impl From<LOSC_AUTO_SWT_PEND_A> for bool {
    #[inline(always)]
    fn from(variant: LOSC_AUTO_SWT_PEND_A) -> Self {
        variant as u8 != 0
    }
}
impl LOSC_AUTO_SWT_PEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LOSC_AUTO_SWT_PEND_A {
        match self.bits {
            false => LOSC_AUTO_SWT_PEND_A::NO_EFFECT,
            true => LOSC_AUTO_SWT_PEND_A::AUTO,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == LOSC_AUTO_SWT_PEND_A::NO_EFFECT
    }
    #[doc = "Auto switch pending, it means LOSC_SRC_SEL is changed from 1 to 0.\n\nSetting 1 to this bit will clear it."]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == LOSC_AUTO_SWT_PEND_A::AUTO
    }
}
#[doc = "Field `losc_auto_swt_pend` writer - LOSC auto switch pending"]
pub type LOSC_AUTO_SWT_PEND_W<'a, REG> = crate::BitWriter1C<'a, REG, LOSC_AUTO_SWT_PEND_A>;
impl<'a, REG> LOSC_AUTO_SWT_PEND_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(LOSC_AUTO_SWT_PEND_A::NO_EFFECT)
    }
    #[doc = "Auto switch pending, it means LOSC_SRC_SEL is changed from 1 to 0.\n\nSetting 1 to this bit will clear it."]
    #[inline(always)]
    pub fn auto(self) -> &'a mut crate::W<REG> {
        self.variant(LOSC_AUTO_SWT_PEND_A::AUTO)
    }
}
#[doc = "Field `ext_losc_sta` reader - Work only when the auto switch function is enabled."]
pub type EXT_LOSC_STA_R = crate::BitReader<EXT_LOSC_STA_A>;
#[doc = "Work only when the auto switch function is enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXT_LOSC_STA_A {
    #[doc = "0: External 32.768 kHz OSC work normally"]
    NORMALLY = 0,
    #[doc = "1: External 32.768 kHz OSC work abnormally"]
    ABNORMALLY = 1,
}
impl From<EXT_LOSC_STA_A> for bool {
    #[inline(always)]
    fn from(variant: EXT_LOSC_STA_A) -> Self {
        variant as u8 != 0
    }
}
impl EXT_LOSC_STA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXT_LOSC_STA_A {
        match self.bits {
            false => EXT_LOSC_STA_A::NORMALLY,
            true => EXT_LOSC_STA_A::ABNORMALLY,
        }
    }
    #[doc = "External 32.768 kHz OSC work normally"]
    #[inline(always)]
    pub fn is_normally(&self) -> bool {
        *self == EXT_LOSC_STA_A::NORMALLY
    }
    #[doc = "External 32.768 kHz OSC work abnormally"]
    #[inline(always)]
    pub fn is_abnormally(&self) -> bool {
        *self == EXT_LOSC_STA_A::ABNORMALLY
    }
}
impl R {
    #[doc = "Bit 0 - Checking LOSC clock source status"]
    #[inline(always)]
    pub fn losc_src_sel_sta(&self) -> LOSC_SRC_SEL_STA_R {
        LOSC_SRC_SEL_STA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LOSC auto switch pending"]
    #[inline(always)]
    pub fn losc_auto_swt_pend(&self) -> LOSC_AUTO_SWT_PEND_R {
        LOSC_AUTO_SWT_PEND_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Work only when the auto switch function is enabled."]
    #[inline(always)]
    pub fn ext_losc_sta(&self) -> EXT_LOSC_STA_R {
        EXT_LOSC_STA_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - LOSC auto switch pending"]
    #[inline(always)]
    #[must_use]
    pub fn losc_auto_swt_pend(&mut self) -> LOSC_AUTO_SWT_PEND_W<LOSC_AUTO_SWT_STA_SPEC> {
        LOSC_AUTO_SWT_PEND_W::new(self, 1)
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
#[doc = "LOSC Auto Switch Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`losc_auto_swt_sta::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`losc_auto_swt_sta::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOSC_AUTO_SWT_STA_SPEC;
impl crate::RegisterSpec for LOSC_AUTO_SWT_STA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`losc_auto_swt_sta::R`](R) reader structure"]
impl crate::Readable for LOSC_AUTO_SWT_STA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`losc_auto_swt_sta::W`](W) writer structure"]
impl crate::Writable for LOSC_AUTO_SWT_STA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x02;
}
#[doc = "`reset()` method sets losc_auto_swt_sta to value 0"]
impl crate::Resettable for LOSC_AUTO_SWT_STA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
