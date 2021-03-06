#[doc = "Register `GP_CH1_CMP_DATA` reader"]
pub struct R(crate::R<GP_CH1_CMP_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GP_CH1_CMP_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GP_CH1_CMP_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GP_CH1_CMP_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GP_CH1_CMP_DATA` writer"]
pub struct W(crate::W<GP_CH1_CMP_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GP_CH1_CMP_DATA_SPEC>;
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
impl From<crate::W<GP_CH1_CMP_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GP_CH1_CMP_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH1_CMP_HIG_DATA` reader - Channel 1 Voltage High Value"]
pub type CH1_CMP_HIG_DATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CH1_CMP_HIG_DATA` writer - Channel 1 Voltage High Value"]
pub type CH1_CMP_HIG_DATA_W<'a> =
    crate::FieldWriter<'a, u32, GP_CH1_CMP_DATA_SPEC, u16, u16, 12, 16>;
#[doc = "Field `CH1_CMP_LOW_DATA` reader - Channel 1 Voltage Low Value"]
pub type CH1_CMP_LOW_DATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CH1_CMP_LOW_DATA` writer - Channel 1 Voltage Low Value"]
pub type CH1_CMP_LOW_DATA_W<'a> =
    crate::FieldWriter<'a, u32, GP_CH1_CMP_DATA_SPEC, u16, u16, 12, 0>;
impl R {
    #[doc = "Bits 16:27 - Channel 1 Voltage High Value"]
    #[inline(always)]
    pub fn ch1_cmp_hig_data(&self) -> CH1_CMP_HIG_DATA_R {
        CH1_CMP_HIG_DATA_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:11 - Channel 1 Voltage Low Value"]
    #[inline(always)]
    pub fn ch1_cmp_low_data(&self) -> CH1_CMP_LOW_DATA_R {
        CH1_CMP_LOW_DATA_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:27 - Channel 1 Voltage High Value"]
    #[inline(always)]
    pub fn ch1_cmp_hig_data(&mut self) -> CH1_CMP_HIG_DATA_W {
        CH1_CMP_HIG_DATA_W::new(self)
    }
    #[doc = "Bits 0:11 - Channel 1 Voltage Low Value"]
    #[inline(always)]
    pub fn ch1_cmp_low_data(&mut self) -> CH1_CMP_LOW_DATA_W {
        CH1_CMP_LOW_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPADC CH1 Compare Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gp_ch1_cmp_data](index.html) module"]
pub struct GP_CH1_CMP_DATA_SPEC;
impl crate::RegisterSpec for GP_CH1_CMP_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gp_ch1_cmp_data::R](R) reader structure"]
impl crate::Readable for GP_CH1_CMP_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gp_ch1_cmp_data::W](W) writer structure"]
impl crate::Writable for GP_CH1_CMP_DATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GP_CH1_CMP_DATA to value 0x0bff_0400"]
impl crate::Resettable for GP_CH1_CMP_DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0bff_0400
    }
}
