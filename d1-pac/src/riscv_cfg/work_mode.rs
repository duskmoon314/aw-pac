#[doc = "Register `work_mode` reader"]
pub type R = crate::R<WORK_MODE_SPEC>;
#[doc = "Field `wm_sta` reader - Work Mode Status"]
pub type WM_STA_R = crate::FieldReader<WM_STA_A>;
#[doc = "Work Mode Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WM_STA_A {
    #[doc = "0: `0`"]
    NORMAL = 0,
    #[doc = "1: `1`"]
    LOW_POWER = 1,
    #[doc = "2: `10`"]
    DEBUG = 2,
}
impl From<WM_STA_A> for u8 {
    #[inline(always)]
    fn from(variant: WM_STA_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WM_STA_A {
    type Ux = u8;
}
impl WM_STA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WM_STA_A> {
        match self.bits {
            0 => Some(WM_STA_A::NORMAL),
            1 => Some(WM_STA_A::LOW_POWER),
            2 => Some(WM_STA_A::DEBUG),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == WM_STA_A::NORMAL
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_low_power(&self) -> bool {
        *self == WM_STA_A::LOW_POWER
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_debug(&self) -> bool {
        *self == WM_STA_A::DEBUG
    }
}
impl R {
    #[doc = "Bits 0:1 - Work Mode Status"]
    #[inline(always)]
    pub fn wm_sta(&self) -> WM_STA_R {
        WM_STA_R::new((self.bits & 3) as u8)
    }
}
#[doc = "Work Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`work_mode::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WORK_MODE_SPEC;
impl crate::RegisterSpec for WORK_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`work_mode::R`](R) reader structure"]
impl crate::Readable for WORK_MODE_SPEC {}
#[doc = "`reset()` method sets work_mode to value 0"]
impl crate::Resettable for WORK_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
