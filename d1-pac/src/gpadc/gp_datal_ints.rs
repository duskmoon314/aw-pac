#[doc = "Register `gp_datal_ints` reader"]
pub struct R(crate::R<GP_DATAL_INTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GP_DATAL_INTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GP_DATAL_INTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GP_DATAL_INTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gp_datal_ints` writer"]
pub struct W(crate::W<GP_DATAL_INTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GP_DATAL_INTS_SPEC>;
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
impl From<crate::W<GP_DATAL_INTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GP_DATAL_INTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ch_low_pengding[0-1]` reader - Channel Voltage Low Available Interrupt Status"]
pub type CH_LOW_PENGDING_R = crate::BitReader<CH_LOW_PENGDING_A>;
#[doc = "Channel Voltage Low Available Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH_LOW_PENGDING_A {
    #[doc = "0: NO Pending IRQ"]
    NO_PENDING = 0,
    #[doc = "1: Channel Voltage Low Available Pending IRQ"]
    PENDING = 1,
}
impl From<CH_LOW_PENGDING_A> for bool {
    #[inline(always)]
    fn from(variant: CH_LOW_PENGDING_A) -> Self {
        variant as u8 != 0
    }
}
impl CH_LOW_PENGDING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH_LOW_PENGDING_A {
        match self.bits {
            false => CH_LOW_PENGDING_A::NO_PENDING,
            true => CH_LOW_PENGDING_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == CH_LOW_PENGDING_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == CH_LOW_PENGDING_A::PENDING
    }
}
#[doc = "Field `ch_low_pengding[0-1]` writer - Channel Voltage Low Available Interrupt Status"]
pub type CH_LOW_PENGDING_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, GP_DATAL_INTS_SPEC, CH_LOW_PENGDING_A, O>;
impl<'a, const O: u8> CH_LOW_PENGDING_W<'a, O> {
    #[doc = "NO Pending IRQ"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut W {
        self.variant(CH_LOW_PENGDING_A::NO_PENDING)
    }
    #[doc = "Channel Voltage Low Available Pending IRQ"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(CH_LOW_PENGDING_A::PENDING)
    }
}
impl R {
    #[doc = "Channel Voltage Low Available Interrupt Status"]
    #[inline(always)]
    pub unsafe fn ch_low_pengding(&self, n: u8) -> CH_LOW_PENGDING_R {
        CH_LOW_PENGDING_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - Channel Voltage Low Available Interrupt Status"]
    #[inline(always)]
    pub fn ch0_low_pengding(&self) -> CH_LOW_PENGDING_R {
        CH_LOW_PENGDING_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Voltage Low Available Interrupt Status"]
    #[inline(always)]
    pub fn ch1_low_pengding(&self) -> CH_LOW_PENGDING_R {
        CH_LOW_PENGDING_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Channel Voltage Low Available Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_low_pengding<const O: u8>(&mut self) -> CH_LOW_PENGDING_W<O> {
        CH_LOW_PENGDING_W::new(self)
    }
    #[doc = "Bit 0 - Channel Voltage Low Available Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn ch0_low_pengding(&mut self) -> CH_LOW_PENGDING_W<0> {
        CH_LOW_PENGDING_W::new(self)
    }
    #[doc = "Bit 1 - Channel Voltage Low Available Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_low_pengding(&mut self) -> CH_LOW_PENGDING_W<1> {
        CH_LOW_PENGDING_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPADC Data Low Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gp_datal_ints](index.html) module"]
pub struct GP_DATAL_INTS_SPEC;
impl crate::RegisterSpec for GP_DATAL_INTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gp_datal_ints::R](R) reader structure"]
impl crate::Readable for GP_DATAL_INTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gp_datal_ints::W](W) writer structure"]
impl crate::Writable for GP_DATAL_INTS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x01;
}
#[doc = "`reset()` method sets gp_datal_ints to value 0"]
impl crate::Resettable for GP_DATAL_INTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
