#[doc = "Register `msgbox_wr_irq_status` reader"]
pub struct R(crate::R<MSGBOX_WR_IRQ_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSGBOX_WR_IRQ_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSGBOX_WR_IRQ_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSGBOX_WR_IRQ_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `msgbox_wr_irq_status` writer"]
pub struct W(crate::W<MSGBOX_WR_IRQ_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSGBOX_WR_IRQ_STATUS_SPEC>;
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
impl From<crate::W<MSGBOX_WR_IRQ_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSGBOX_WR_IRQ_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `transmit_mq_irq_pend[0-3]` reader - Transmit Channel\\[i\\] Interrupt Pending"]
pub type TRANSMIT_MQ_IRQ_PEND_R = crate::BitReader<TRANSMIT_MQ_IRQ_PEND_A>;
#[doc = "Transmit Channel\\[i\\] Interrupt Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRANSMIT_MQ_IRQ_PEND_A {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Pending"]
    PENDING = 1,
}
impl From<TRANSMIT_MQ_IRQ_PEND_A> for bool {
    #[inline(always)]
    fn from(variant: TRANSMIT_MQ_IRQ_PEND_A) -> Self {
        variant as u8 != 0
    }
}
impl TRANSMIT_MQ_IRQ_PEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRANSMIT_MQ_IRQ_PEND_A {
        match self.bits {
            false => TRANSMIT_MQ_IRQ_PEND_A::NO_EFFECT,
            true => TRANSMIT_MQ_IRQ_PEND_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TRANSMIT_MQ_IRQ_PEND_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == TRANSMIT_MQ_IRQ_PEND_A::PENDING
    }
}
#[doc = "Field `transmit_mq_irq_pend[0-3]` writer - Transmit Channel\\[i\\] Interrupt Pending"]
pub type TRANSMIT_MQ_IRQ_PEND_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, MSGBOX_WR_IRQ_STATUS_SPEC, TRANSMIT_MQ_IRQ_PEND_A, O>;
impl<'a, const O: u8> TRANSMIT_MQ_IRQ_PEND_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TRANSMIT_MQ_IRQ_PEND_A::NO_EFFECT)
    }
    #[doc = "Pending"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(TRANSMIT_MQ_IRQ_PEND_A::PENDING)
    }
}
impl R {
    #[doc = "Transmit Channel\\[i\\] Interrupt Pending"]
    #[inline(always)]
    pub unsafe fn transmit_mq_irq_pend(&self, n: u8) -> TRANSMIT_MQ_IRQ_PEND_R {
        TRANSMIT_MQ_IRQ_PEND_R::new(((self.bits >> (n * 2)) & 1) != 0)
    }
    #[doc = "Bit 0 - Transmit Channel\\[i\\] Interrupt Pending"]
    #[inline(always)]
    pub fn transmit_mq0_irq_pend(&self) -> TRANSMIT_MQ_IRQ_PEND_R {
        TRANSMIT_MQ_IRQ_PEND_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Channel\\[i\\] Interrupt Pending"]
    #[inline(always)]
    pub fn transmit_mq1_irq_pend(&self) -> TRANSMIT_MQ_IRQ_PEND_R {
        TRANSMIT_MQ_IRQ_PEND_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit Channel\\[i\\] Interrupt Pending"]
    #[inline(always)]
    pub fn transmit_mq2_irq_pend(&self) -> TRANSMIT_MQ_IRQ_PEND_R {
        TRANSMIT_MQ_IRQ_PEND_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit Channel\\[i\\] Interrupt Pending"]
    #[inline(always)]
    pub fn transmit_mq3_irq_pend(&self) -> TRANSMIT_MQ_IRQ_PEND_R {
        TRANSMIT_MQ_IRQ_PEND_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Transmit Channel\\[i\\] Interrupt Pending"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn transmit_mq_irq_pend<const O: u8>(&mut self) -> TRANSMIT_MQ_IRQ_PEND_W<O> {
        TRANSMIT_MQ_IRQ_PEND_W::new(self)
    }
    #[doc = "Bit 0 - Transmit Channel\\[i\\] Interrupt Pending"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_mq0_irq_pend(&mut self) -> TRANSMIT_MQ_IRQ_PEND_W<0> {
        TRANSMIT_MQ_IRQ_PEND_W::new(self)
    }
    #[doc = "Bit 2 - Transmit Channel\\[i\\] Interrupt Pending"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_mq1_irq_pend(&mut self) -> TRANSMIT_MQ_IRQ_PEND_W<2> {
        TRANSMIT_MQ_IRQ_PEND_W::new(self)
    }
    #[doc = "Bit 4 - Transmit Channel\\[i\\] Interrupt Pending"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_mq2_irq_pend(&mut self) -> TRANSMIT_MQ_IRQ_PEND_W<4> {
        TRANSMIT_MQ_IRQ_PEND_W::new(self)
    }
    #[doc = "Bit 6 - Transmit Channel\\[i\\] Interrupt Pending"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_mq3_irq_pend(&mut self) -> TRANSMIT_MQ_IRQ_PEND_W<6> {
        TRANSMIT_MQ_IRQ_PEND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Message Box Write Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msgbox_wr_irq_status](index.html) module"]
pub struct MSGBOX_WR_IRQ_STATUS_SPEC;
impl crate::RegisterSpec for MSGBOX_WR_IRQ_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msgbox_wr_irq_status::R](R) reader structure"]
impl crate::Readable for MSGBOX_WR_IRQ_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [msgbox_wr_irq_status::W](W) writer structure"]
impl crate::Writable for MSGBOX_WR_IRQ_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x01;
}
#[doc = "`reset()` method sets msgbox_wr_irq_status to value 0"]
impl crate::Resettable for MSGBOX_WR_IRQ_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
