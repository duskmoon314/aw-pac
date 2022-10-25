#[doc = "Register `gp_ch0_cmp_data` reader"]
pub struct R(crate::R<GP_CH0_CMP_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GP_CH0_CMP_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GP_CH0_CMP_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GP_CH0_CMP_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gp_ch0_cmp_data` writer"]
pub struct W(crate::W<GP_CH0_CMP_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GP_CH0_CMP_DATA_SPEC>;
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
impl From<crate::W<GP_CH0_CMP_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GP_CH0_CMP_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ch0_cmp_low_data` reader - Channel 0 Voltage Low Value"]
pub type CH0_CMP_LOW_DATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ch0_cmp_low_data` writer - Channel 0 Voltage Low Value"]
pub type CH0_CMP_LOW_DATA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GP_CH0_CMP_DATA_SPEC, u16, u16, 12, O>;
#[doc = "Field `ch0_cmp_hig_data` reader - Channel 0 Voltage High Value"]
pub type CH0_CMP_HIG_DATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ch0_cmp_hig_data` writer - Channel 0 Voltage High Value"]
pub type CH0_CMP_HIG_DATA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GP_CH0_CMP_DATA_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - Channel 0 Voltage Low Value"]
    #[inline(always)]
    pub fn ch0_cmp_low_data(&self) -> CH0_CMP_LOW_DATA_R {
        CH0_CMP_LOW_DATA_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Channel 0 Voltage High Value"]
    #[inline(always)]
    pub fn ch0_cmp_hig_data(&self) -> CH0_CMP_HIG_DATA_R {
        CH0_CMP_HIG_DATA_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Channel 0 Voltage Low Value"]
    #[inline(always)]
    #[must_use]
    pub fn ch0_cmp_low_data(&mut self) -> CH0_CMP_LOW_DATA_W<0> {
        CH0_CMP_LOW_DATA_W::new(self)
    }
    #[doc = "Bits 16:27 - Channel 0 Voltage High Value"]
    #[inline(always)]
    #[must_use]
    pub fn ch0_cmp_hig_data(&mut self) -> CH0_CMP_HIG_DATA_W<16> {
        CH0_CMP_HIG_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPADC CH0 Compare Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gp_ch0_cmp_data](index.html) module"]
pub struct GP_CH0_CMP_DATA_SPEC;
impl crate::RegisterSpec for GP_CH0_CMP_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gp_ch0_cmp_data::R](R) reader structure"]
impl crate::Readable for GP_CH0_CMP_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gp_ch0_cmp_data::W](W) writer structure"]
impl crate::Writable for GP_CH0_CMP_DATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gp_ch0_cmp_data to value 0x0bff_0400"]
impl crate::Resettable for GP_CH0_CMP_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0x0bff_0400;
}
