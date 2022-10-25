#[doc = "Register `msgbox_rd_irq_en` reader"]
pub struct R(crate::R<MSGBOX_RD_IRQ_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSGBOX_RD_IRQ_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSGBOX_RD_IRQ_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSGBOX_RD_IRQ_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `msgbox_rd_irq_en` writer"]
pub struct W(crate::W<MSGBOX_RD_IRQ_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSGBOX_RD_IRQ_EN_SPEC>;
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
impl From<crate::W<MSGBOX_RD_IRQ_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSGBOX_RD_IRQ_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reception_mq_irq_en[0-3]` reader - Reception Channel\\[i\\] Interrupt Enable"]
pub type RECEPTION_MQ_IRQ_EN_R = crate::BitReader<RECEPTION_MQ_IRQ_EN_A>;
#[doc = "Reception Channel\\[i\\] Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RECEPTION_MQ_IRQ_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<RECEPTION_MQ_IRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RECEPTION_MQ_IRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl RECEPTION_MQ_IRQ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEPTION_MQ_IRQ_EN_A {
        match self.bits {
            false => RECEPTION_MQ_IRQ_EN_A::DISABLE,
            true => RECEPTION_MQ_IRQ_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RECEPTION_MQ_IRQ_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RECEPTION_MQ_IRQ_EN_A::ENABLE
    }
}
#[doc = "Field `reception_mq_irq_en[0-3]` writer - Reception Channel\\[i\\] Interrupt Enable"]
pub type RECEPTION_MQ_IRQ_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MSGBOX_RD_IRQ_EN_SPEC, RECEPTION_MQ_IRQ_EN_A, O>;
impl<'a, const O: u8> RECEPTION_MQ_IRQ_EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RECEPTION_MQ_IRQ_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RECEPTION_MQ_IRQ_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Reception Channel\\[i\\] Interrupt Enable"]
    #[inline(always)]
    pub unsafe fn reception_mq_irq_en(&self, n: u8) -> RECEPTION_MQ_IRQ_EN_R {
        RECEPTION_MQ_IRQ_EN_R::new(((self.bits >> (n * 2)) & 1) != 0)
    }
    #[doc = "Bit 0 - Reception Channel\\[i\\] Interrupt Enable"]
    #[inline(always)]
    pub fn reception_mq0_irq_en(&self) -> RECEPTION_MQ_IRQ_EN_R {
        RECEPTION_MQ_IRQ_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Reception Channel\\[i\\] Interrupt Enable"]
    #[inline(always)]
    pub fn reception_mq1_irq_en(&self) -> RECEPTION_MQ_IRQ_EN_R {
        RECEPTION_MQ_IRQ_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Reception Channel\\[i\\] Interrupt Enable"]
    #[inline(always)]
    pub fn reception_mq2_irq_en(&self) -> RECEPTION_MQ_IRQ_EN_R {
        RECEPTION_MQ_IRQ_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Reception Channel\\[i\\] Interrupt Enable"]
    #[inline(always)]
    pub fn reception_mq3_irq_en(&self) -> RECEPTION_MQ_IRQ_EN_R {
        RECEPTION_MQ_IRQ_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Reception Channel\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn reception_mq_irq_en<const O: u8>(&mut self) -> RECEPTION_MQ_IRQ_EN_W<O> {
        RECEPTION_MQ_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 0 - Reception Channel\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn reception_mq0_irq_en(&mut self) -> RECEPTION_MQ_IRQ_EN_W<0> {
        RECEPTION_MQ_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 2 - Reception Channel\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn reception_mq1_irq_en(&mut self) -> RECEPTION_MQ_IRQ_EN_W<2> {
        RECEPTION_MQ_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 4 - Reception Channel\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn reception_mq2_irq_en(&mut self) -> RECEPTION_MQ_IRQ_EN_W<4> {
        RECEPTION_MQ_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 6 - Reception Channel\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn reception_mq3_irq_en(&mut self) -> RECEPTION_MQ_IRQ_EN_W<6> {
        RECEPTION_MQ_IRQ_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Message Box Read Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msgbox_rd_irq_en](index.html) module"]
pub struct MSGBOX_RD_IRQ_EN_SPEC;
impl crate::RegisterSpec for MSGBOX_RD_IRQ_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msgbox_rd_irq_en::R](R) reader structure"]
impl crate::Readable for MSGBOX_RD_IRQ_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [msgbox_rd_irq_en::W](W) writer structure"]
impl crate::Writable for MSGBOX_RD_IRQ_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets msgbox_rd_irq_en to value 0"]
impl crate::Resettable for MSGBOX_RD_IRQ_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
