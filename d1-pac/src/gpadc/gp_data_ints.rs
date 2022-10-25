#[doc = "Register `gp_data_ints` reader"]
pub struct R(crate::R<GP_DATA_INTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GP_DATA_INTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GP_DATA_INTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GP_DATA_INTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gp_data_ints` writer"]
pub struct W(crate::W<GP_DATA_INTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GP_DATA_INTS_SPEC>;
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
impl From<crate::W<GP_DATA_INTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GP_DATA_INTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ch_data_pengding[0-1]` reader - Channel Data Available Interrupt Status"]
pub type CH_DATA_PENGDING_R = crate::BitReader<CH_DATA_PENGDING_A>;
#[doc = "Channel Data Available Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH_DATA_PENGDING_A {
    #[doc = "0: No Pending IRQ"]
    NO_PENDING = 0,
    #[doc = "1: Channel Data Available Pending IRQ"]
    CHANNEL = 1,
}
impl From<CH_DATA_PENGDING_A> for bool {
    #[inline(always)]
    fn from(variant: CH_DATA_PENGDING_A) -> Self {
        variant as u8 != 0
    }
}
impl CH_DATA_PENGDING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH_DATA_PENGDING_A {
        match self.bits {
            false => CH_DATA_PENGDING_A::NO_PENDING,
            true => CH_DATA_PENGDING_A::CHANNEL,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == CH_DATA_PENGDING_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `CHANNEL`"]
    #[inline(always)]
    pub fn is_channel(&self) -> bool {
        *self == CH_DATA_PENGDING_A::CHANNEL
    }
}
#[doc = "Field `ch_data_pengding[0-1]` writer - Channel Data Available Interrupt Status"]
pub type CH_DATA_PENGDING_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, GP_DATA_INTS_SPEC, CH_DATA_PENGDING_A, O>;
impl<'a, const O: u8> CH_DATA_PENGDING_W<'a, O> {
    #[doc = "No Pending IRQ"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut W {
        self.variant(CH_DATA_PENGDING_A::NO_PENDING)
    }
    #[doc = "Channel Data Available Pending IRQ"]
    #[inline(always)]
    pub fn channel(self) -> &'a mut W {
        self.variant(CH_DATA_PENGDING_A::CHANNEL)
    }
}
impl R {
    #[doc = "Channel Data Available Interrupt Status"]
    #[inline(always)]
    pub unsafe fn ch_data_pengding(&self, n: u8) -> CH_DATA_PENGDING_R {
        CH_DATA_PENGDING_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - Channel Data Available Interrupt Status"]
    #[inline(always)]
    pub fn ch0_data_pengding(&self) -> CH_DATA_PENGDING_R {
        CH_DATA_PENGDING_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Data Available Interrupt Status"]
    #[inline(always)]
    pub fn ch1_data_pengding(&self) -> CH_DATA_PENGDING_R {
        CH_DATA_PENGDING_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Channel Data Available Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_data_pengding<const O: u8>(&mut self) -> CH_DATA_PENGDING_W<O> {
        CH_DATA_PENGDING_W::new(self)
    }
    #[doc = "Bit 0 - Channel Data Available Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn ch0_data_pengding(&mut self) -> CH_DATA_PENGDING_W<0> {
        CH_DATA_PENGDING_W::new(self)
    }
    #[doc = "Bit 1 - Channel Data Available Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_data_pengding(&mut self) -> CH_DATA_PENGDING_W<1> {
        CH_DATA_PENGDING_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPADC Data Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gp_data_ints](index.html) module"]
pub struct GP_DATA_INTS_SPEC;
impl crate::RegisterSpec for GP_DATA_INTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gp_data_ints::R](R) reader structure"]
impl crate::Readable for GP_DATA_INTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gp_data_ints::W](W) writer structure"]
impl crate::Writable for GP_DATA_INTS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x01;
}
#[doc = "`reset()` method sets gp_data_ints to value 0"]
impl crate::Resettable for GP_DATA_INTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
