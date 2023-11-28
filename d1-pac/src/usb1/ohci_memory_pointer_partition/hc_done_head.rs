#[doc = "Register `hc_done_head` reader"]
pub type R = crate::R<HC_DONE_HEAD_SPEC>;
#[doc = "Register `hc_done_head` writer"]
pub type W = crate::W<HC_DONE_HEAD_SPEC>;
#[doc = "Field `dh_3_0` reader - Because the general TD length is 16 bytes, the memory structure for the TD must be aligned to a 16-byte boundary. So the lower bits in the PCED, through bit 0 to bit 3 must be zero in this field."]
pub type DH_3_0_R = crate::FieldReader;
#[doc = "Field `dh_31_4` reader - When a TD is completed, HC writes the content of HcDoneHead to the NextTD field of the TD. HC then overwrites the content of HcDoneHead with the address of this TD. This is set to zero whenever HC writes the content of this register to HCCA. It also sets the WritebackDoneHead of HcInterruptStatus."]
pub type DH_31_4_R = crate::FieldReader<u32>;
#[doc = "Field `dh_31_4` writer - When a TD is completed, HC writes the content of HcDoneHead to the NextTD field of the TD. HC then overwrites the content of HcDoneHead with the address of this TD. This is set to zero whenever HC writes the content of this register to HCCA. It also sets the WritebackDoneHead of HcInterruptStatus."]
pub type DH_31_4_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:3 - Because the general TD length is 16 bytes, the memory structure for the TD must be aligned to a 16-byte boundary. So the lower bits in the PCED, through bit 0 to bit 3 must be zero in this field."]
    #[inline(always)]
    pub fn dh_3_0(&self) -> DH_3_0_R {
        DH_3_0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31 - When a TD is completed, HC writes the content of HcDoneHead to the NextTD field of the TD. HC then overwrites the content of HcDoneHead with the address of this TD. This is set to zero whenever HC writes the content of this register to HCCA. It also sets the WritebackDoneHead of HcInterruptStatus."]
    #[inline(always)]
    pub fn dh_31_4(&self) -> DH_31_4_R {
        DH_31_4_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:31 - When a TD is completed, HC writes the content of HcDoneHead to the NextTD field of the TD. HC then overwrites the content of HcDoneHead with the address of this TD. This is set to zero whenever HC writes the content of this register to HCCA. It also sets the WritebackDoneHead of HcInterruptStatus."]
    #[inline(always)]
    #[must_use]
    pub fn dh_31_4(&mut self) -> DH_31_4_W<HC_DONE_HEAD_SPEC> {
        DH_31_4_W::new(self, 4)
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
#[doc = "OHCI Done Head Base\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc_done_head::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc_done_head::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HC_DONE_HEAD_SPEC;
impl crate::RegisterSpec for HC_DONE_HEAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hc_done_head::R`](R) reader structure"]
impl crate::Readable for HC_DONE_HEAD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hc_done_head::W`](W) writer structure"]
impl crate::Writable for HC_DONE_HEAD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hc_done_head to value 0"]
impl crate::Resettable for HC_DONE_HEAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
