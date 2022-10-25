#[doc = "Register `emac_mii_data` reader"]
pub struct R(crate::R<EMAC_MII_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMAC_MII_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMAC_MII_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMAC_MII_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `emac_mii_data` writer"]
pub struct W(crate::W<EMAC_MII_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMAC_MII_DATA_SPEC>;
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
impl From<crate::W<EMAC_MII_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMAC_MII_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `mii_data` reader - "]
pub type MII_DATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `mii_data` writer - "]
pub type MII_DATA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EMAC_MII_DATA_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn mii_data(&self) -> MII_DATA_R {
        MII_DATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn mii_data(&mut self) -> MII_DATA_W<0> {
        MII_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EMAC Management Interface Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emac_mii_data](index.html) module"]
pub struct EMAC_MII_DATA_SPEC;
impl crate::RegisterSpec for EMAC_MII_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emac_mii_data::R](R) reader structure"]
impl crate::Readable for EMAC_MII_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emac_mii_data::W](W) writer structure"]
impl crate::Writable for EMAC_MII_DATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets emac_mii_data to value 0"]
impl crate::Resettable for EMAC_MII_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
