#[doc = "Register `hc_bulk_current_ed` reader"]
pub struct R(crate::R<HC_BULK_CURRENT_ED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HC_BULK_CURRENT_ED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HC_BULK_CURRENT_ED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HC_BULK_CURRENT_ED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `hc_bulk_current_ed` writer"]
pub struct W(crate::W<HC_BULK_CURRENT_ED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HC_BULK_CURRENT_ED_SPEC>;
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
impl From<crate::W<HC_BULK_CURRENT_ED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HC_BULK_CURRENT_ED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `bced_3_0` reader - Because the general TD length is 16 bytes, the memory structure for the TD must be aligned to a 16-byte boundary. So the lower bits in the PCED, through bit 0 to bit 3 must be zero in this field."]
pub type BCED_3_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `bced_31_4` reader - This is advanced to the next ED after the HC has served the present one. HC continues processing the list from where it left off in the last Frame. When it reaches the end of the Bulk list, HC checks the ControlListFilled of HcControl. If set, it copies the content of HcBulkHeadED to HcBulkCurrentED and clears the bit. If it is not set, it does nothing. HCD is only allowed to modify this register when the BulkListEnable of HcControl is cleared. When set, the HCD only reads the instantaneous value of this register. This is initially set to zero to indicate the end of the Bulk list."]
pub type BCED_31_4_R = crate::FieldReader<u32, u32>;
#[doc = "Field `bced_31_4` writer - This is advanced to the next ED after the HC has served the present one. HC continues processing the list from where it left off in the last Frame. When it reaches the end of the Bulk list, HC checks the ControlListFilled of HcControl. If set, it copies the content of HcBulkHeadED to HcBulkCurrentED and clears the bit. If it is not set, it does nothing. HCD is only allowed to modify this register when the BulkListEnable of HcControl is cleared. When set, the HCD only reads the instantaneous value of this register. This is initially set to zero to indicate the end of the Bulk list."]
pub type BCED_31_4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HC_BULK_CURRENT_ED_SPEC, u32, u32, 28, O>;
impl R {
    #[doc = "Bits 0:3 - Because the general TD length is 16 bytes, the memory structure for the TD must be aligned to a 16-byte boundary. So the lower bits in the PCED, through bit 0 to bit 3 must be zero in this field."]
    #[inline(always)]
    pub fn bced_3_0(&self) -> BCED_3_0_R {
        BCED_3_0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31 - This is advanced to the next ED after the HC has served the present one. HC continues processing the list from where it left off in the last Frame. When it reaches the end of the Bulk list, HC checks the ControlListFilled of HcControl. If set, it copies the content of HcBulkHeadED to HcBulkCurrentED and clears the bit. If it is not set, it does nothing. HCD is only allowed to modify this register when the BulkListEnable of HcControl is cleared. When set, the HCD only reads the instantaneous value of this register. This is initially set to zero to indicate the end of the Bulk list."]
    #[inline(always)]
    pub fn bced_31_4(&self) -> BCED_31_4_R {
        BCED_31_4_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:31 - This is advanced to the next ED after the HC has served the present one. HC continues processing the list from where it left off in the last Frame. When it reaches the end of the Bulk list, HC checks the ControlListFilled of HcControl. If set, it copies the content of HcBulkHeadED to HcBulkCurrentED and clears the bit. If it is not set, it does nothing. HCD is only allowed to modify this register when the BulkListEnable of HcControl is cleared. When set, the HCD only reads the instantaneous value of this register. This is initially set to zero to indicate the end of the Bulk list."]
    #[inline(always)]
    #[must_use]
    pub fn bced_31_4(&mut self) -> BCED_31_4_W<4> {
        BCED_31_4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OHCI Bulk Current ED Base\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc_bulk_current_ed](index.html) module"]
pub struct HC_BULK_CURRENT_ED_SPEC;
impl crate::RegisterSpec for HC_BULK_CURRENT_ED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hc_bulk_current_ed::R](R) reader structure"]
impl crate::Readable for HC_BULK_CURRENT_ED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hc_bulk_current_ed::W](W) writer structure"]
impl crate::Writable for HC_BULK_CURRENT_ED_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hc_bulk_current_ed to value 0"]
impl crate::Resettable for HC_BULK_CURRENT_ED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
