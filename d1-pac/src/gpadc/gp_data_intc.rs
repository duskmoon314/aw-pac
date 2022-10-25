#[doc = "Register `gp_data_intc` reader"]
pub struct R(crate::R<GP_DATA_INTC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GP_DATA_INTC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GP_DATA_INTC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GP_DATA_INTC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gp_data_intc` writer"]
pub struct W(crate::W<GP_DATA_INTC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GP_DATA_INTC_SPEC>;
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
impl From<crate::W<GP_DATA_INTC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GP_DATA_INTC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ch_data_irq_en[0-1]` reader - Channel Data Interrupt Enable"]
pub type CH_DATA_IRQ_EN_R = crate::BitReader<CH_DATA_IRQ_EN_A>;
#[doc = "Channel Data Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH_DATA_IRQ_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<CH_DATA_IRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CH_DATA_IRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl CH_DATA_IRQ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH_DATA_IRQ_EN_A {
        match self.bits {
            false => CH_DATA_IRQ_EN_A::DISABLE,
            true => CH_DATA_IRQ_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH_DATA_IRQ_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CH_DATA_IRQ_EN_A::ENABLE
    }
}
#[doc = "Field `ch_data_irq_en[0-1]` writer - Channel Data Interrupt Enable"]
pub type CH_DATA_IRQ_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GP_DATA_INTC_SPEC, CH_DATA_IRQ_EN_A, O>;
impl<'a, const O: u8> CH_DATA_IRQ_EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH_DATA_IRQ_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CH_DATA_IRQ_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Channel Data Interrupt Enable"]
    #[inline(always)]
    pub unsafe fn ch_data_irq_en(&self, n: u8) -> CH_DATA_IRQ_EN_R {
        CH_DATA_IRQ_EN_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - Channel Data Interrupt Enable"]
    #[inline(always)]
    pub fn ch0_data_irq_en(&self) -> CH_DATA_IRQ_EN_R {
        CH_DATA_IRQ_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Data Interrupt Enable"]
    #[inline(always)]
    pub fn ch1_data_irq_en(&self) -> CH_DATA_IRQ_EN_R {
        CH_DATA_IRQ_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Channel Data Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_data_irq_en<const O: u8>(&mut self) -> CH_DATA_IRQ_EN_W<O> {
        CH_DATA_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 0 - Channel Data Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0_data_irq_en(&mut self) -> CH_DATA_IRQ_EN_W<0> {
        CH_DATA_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 1 - Channel Data Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_data_irq_en(&mut self) -> CH_DATA_IRQ_EN_W<1> {
        CH_DATA_IRQ_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPADC Data Interrupt Configure Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gp_data_intc](index.html) module"]
pub struct GP_DATA_INTC_SPEC;
impl crate::RegisterSpec for GP_DATA_INTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gp_data_intc::R](R) reader structure"]
impl crate::Readable for GP_DATA_INTC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gp_data_intc::W](W) writer structure"]
impl crate::Writable for GP_DATA_INTC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gp_data_intc to value 0"]
impl crate::Resettable for GP_DATA_INTC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
