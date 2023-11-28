#[doc = "Register `hc_bulk_current_ed` reader"]
pub type R = crate::R<HC_BULK_CURRENT_ED_SPEC>;
#[doc = "Register `hc_bulk_current_ed` writer"]
pub type W = crate::W<HC_BULK_CURRENT_ED_SPEC>;
#[doc = "Field `bced_3_0` reader - Because the general TD length is 16 bytes, the memory structure for the TD must be aligned to a 16-byte boundary. So the lower bits in the PCED, through bit 0 to bit 3 must be zero in this field."]
pub type BCED_3_0_R = crate::FieldReader;
#[doc = "Field `bced_31_4` reader - This is advanced to the next ED after the HC has served the present one. HC continues processing the list from where it left off in the last Frame. When it reaches the end of the Bulk list, HC checks the ControlListFilled of HcControl. If set, it copies the content of HcBulkHeadED to HcBulkCurrentED and clears the bit. If it is not set, it does nothing. HCD is only allowed to modify this register when the BulkListEnable of HcControl is cleared. When set, the HCD only reads the instantaneous value of this register. This is initially set to zero to indicate the end of the Bulk list."]
pub type BCED_31_4_R = crate::FieldReader<u32>;
#[doc = "Field `bced_31_4` writer - This is advanced to the next ED after the HC has served the present one. HC continues processing the list from where it left off in the last Frame. When it reaches the end of the Bulk list, HC checks the ControlListFilled of HcControl. If set, it copies the content of HcBulkHeadED to HcBulkCurrentED and clears the bit. If it is not set, it does nothing. HCD is only allowed to modify this register when the BulkListEnable of HcControl is cleared. When set, the HCD only reads the instantaneous value of this register. This is initially set to zero to indicate the end of the Bulk list."]
pub type BCED_31_4_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
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
    pub fn bced_31_4(&mut self) -> BCED_31_4_W<HC_BULK_CURRENT_ED_SPEC> {
        BCED_31_4_W::new(self, 4)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "OHCI Bulk Current ED Base\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc_bulk_current_ed::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc_bulk_current_ed::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HC_BULK_CURRENT_ED_SPEC;
impl crate::RegisterSpec for HC_BULK_CURRENT_ED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hc_bulk_current_ed::R`](R) reader structure"]
impl crate::Readable for HC_BULK_CURRENT_ED_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hc_bulk_current_ed::W`](W) writer structure"]
impl crate::Writable for HC_BULK_CURRENT_ED_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hc_bulk_current_ed to value 0"]
impl crate::Resettable for HC_BULK_CURRENT_ED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
