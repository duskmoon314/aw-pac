#[doc = "Register `msgbox_fifo_status%s` reader"]
pub type R = crate::R<MSGBOX_FIFO_STATUS_SPEC>;
#[doc = "Field `fifo_not_ava_flag` reader - FIFO Not Available Flag"]
pub type FIFO_NOT_AVA_FLAG_R = crate::BitReader<FIFO_NOT_AVA_FLAG_A>;
#[doc = "FIFO Not Available Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFO_NOT_AVA_FLAG_A {
    #[doc = "0: The Message FIFO queue empty level reaches the configured threshold"]
    AVAILABLE = 0,
    #[doc = "1: The Message FIFO queue empty level does not reach the configured threshold"]
    NOT_AVAILABLE = 1,
}
impl From<FIFO_NOT_AVA_FLAG_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_NOT_AVA_FLAG_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFO_NOT_AVA_FLAG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FIFO_NOT_AVA_FLAG_A {
        match self.bits {
            false => FIFO_NOT_AVA_FLAG_A::AVAILABLE,
            true => FIFO_NOT_AVA_FLAG_A::NOT_AVAILABLE,
        }
    }
    #[doc = "The Message FIFO queue empty level reaches the configured threshold"]
    #[inline(always)]
    pub fn is_available(&self) -> bool {
        *self == FIFO_NOT_AVA_FLAG_A::AVAILABLE
    }
    #[doc = "The Message FIFO queue empty level does not reach the configured threshold"]
    #[inline(always)]
    pub fn is_not_available(&self) -> bool {
        *self == FIFO_NOT_AVA_FLAG_A::NOT_AVAILABLE
    }
}
impl R {
    #[doc = "Bit 0 - FIFO Not Available Flag"]
    #[inline(always)]
    pub fn fifo_not_ava_flag(&self) -> FIFO_NOT_AVA_FLAG_R {
        FIFO_NOT_AVA_FLAG_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Message Box FIFO Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msgbox_fifo_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSGBOX_FIFO_STATUS_SPEC;
impl crate::RegisterSpec for MSGBOX_FIFO_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msgbox_fifo_status::R`](R) reader structure"]
impl crate::Readable for MSGBOX_FIFO_STATUS_SPEC {}
#[doc = "`reset()` method sets msgbox_fifo_status%s to value 0"]
impl crate::Resettable for MSGBOX_FIFO_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
