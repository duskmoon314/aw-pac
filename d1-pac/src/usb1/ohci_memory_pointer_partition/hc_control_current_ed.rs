#[doc = "Register `hc_control_current_ed` reader"]
pub type R = crate::R<HC_CONTROL_CURRENT_ED_SPEC>;
#[doc = "Register `hc_control_current_ed` writer"]
pub type W = crate::W<HC_CONTROL_CURRENT_ED_SPEC>;
#[doc = "Field `cced_3_0` reader - Because the general TD length is 16 bytes, the memory structure for the TD must be aligned to a 16-byte boundary. So the lower bits in the PCED, through bit 0 to bit 3 must be zero in this field."]
pub type CCED_3_0_R = crate::FieldReader;
#[doc = "Field `cced_31_4` reader - The pointer is advanced to the next ED after serving the present one. HC will continue processing the list from where it left off in the last Frame. When it reaches the end of the Control list, HC checks the ControlListFilled of in HcCommandStatus. If set, it copies the content of HcControlHeadED to HcControlCurrentED and clears the bit. If not set, it does nothing.\n\nHCD is allowed to modify this register only when the ControlListEnable of HcControl is cleared. When set, HCD only reads the instantaneous value of this register. Initially, this is set to zero to indicate the end of the Control list."]
pub type CCED_31_4_R = crate::FieldReader<u32>;
#[doc = "Field `cced_31_4` writer - The pointer is advanced to the next ED after serving the present one. HC will continue processing the list from where it left off in the last Frame. When it reaches the end of the Control list, HC checks the ControlListFilled of in HcCommandStatus. If set, it copies the content of HcControlHeadED to HcControlCurrentED and clears the bit. If not set, it does nothing.\n\nHCD is allowed to modify this register only when the ControlListEnable of HcControl is cleared. When set, HCD only reads the instantaneous value of this register. Initially, this is set to zero to indicate the end of the Control list."]
pub type CCED_31_4_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:3 - Because the general TD length is 16 bytes, the memory structure for the TD must be aligned to a 16-byte boundary. So the lower bits in the PCED, through bit 0 to bit 3 must be zero in this field."]
    #[inline(always)]
    pub fn cced_3_0(&self) -> CCED_3_0_R {
        CCED_3_0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31 - The pointer is advanced to the next ED after serving the present one. HC will continue processing the list from where it left off in the last Frame. When it reaches the end of the Control list, HC checks the ControlListFilled of in HcCommandStatus. If set, it copies the content of HcControlHeadED to HcControlCurrentED and clears the bit. If not set, it does nothing.\n\nHCD is allowed to modify this register only when the ControlListEnable of HcControl is cleared. When set, HCD only reads the instantaneous value of this register. Initially, this is set to zero to indicate the end of the Control list."]
    #[inline(always)]
    pub fn cced_31_4(&self) -> CCED_31_4_R {
        CCED_31_4_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:31 - The pointer is advanced to the next ED after serving the present one. HC will continue processing the list from where it left off in the last Frame. When it reaches the end of the Control list, HC checks the ControlListFilled of in HcCommandStatus. If set, it copies the content of HcControlHeadED to HcControlCurrentED and clears the bit. If not set, it does nothing.\n\nHCD is allowed to modify this register only when the ControlListEnable of HcControl is cleared. When set, HCD only reads the instantaneous value of this register. Initially, this is set to zero to indicate the end of the Control list."]
    #[inline(always)]
    #[must_use]
    pub fn cced_31_4(&mut self) -> CCED_31_4_W<HC_CONTROL_CURRENT_ED_SPEC> {
        CCED_31_4_W::new(self, 4)
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
#[doc = "OHCI Control Current ED Base\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc_control_current_ed::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc_control_current_ed::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HC_CONTROL_CURRENT_ED_SPEC;
impl crate::RegisterSpec for HC_CONTROL_CURRENT_ED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hc_control_current_ed::R`](R) reader structure"]
impl crate::Readable for HC_CONTROL_CURRENT_ED_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hc_control_current_ed::W`](W) writer structure"]
impl crate::Writable for HC_CONTROL_CURRENT_ED_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hc_control_current_ed to value 0"]
impl crate::Resettable for HC_CONTROL_CURRENT_ED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
