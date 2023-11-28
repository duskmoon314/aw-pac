#[doc = "Register `gp_ch1_cmp_data` reader"]
pub type R = crate::R<GP_CH1_CMP_DATA_SPEC>;
#[doc = "Register `gp_ch1_cmp_data` writer"]
pub type W = crate::W<GP_CH1_CMP_DATA_SPEC>;
#[doc = "Field `ch1_cmp_low_data` reader - Channel 1 Voltage Low Value"]
pub type CH1_CMP_LOW_DATA_R = crate::FieldReader<u16>;
#[doc = "Field `ch1_cmp_low_data` writer - Channel 1 Voltage Low Value"]
pub type CH1_CMP_LOW_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ch1_cmp_hig_data` reader - Channel 1 Voltage High Value"]
pub type CH1_CMP_HIG_DATA_R = crate::FieldReader<u16>;
#[doc = "Field `ch1_cmp_hig_data` writer - Channel 1 Voltage High Value"]
pub type CH1_CMP_HIG_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Channel 1 Voltage Low Value"]
    #[inline(always)]
    pub fn ch1_cmp_low_data(&self) -> CH1_CMP_LOW_DATA_R {
        CH1_CMP_LOW_DATA_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Channel 1 Voltage High Value"]
    #[inline(always)]
    pub fn ch1_cmp_hig_data(&self) -> CH1_CMP_HIG_DATA_R {
        CH1_CMP_HIG_DATA_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Channel 1 Voltage Low Value"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_cmp_low_data(&mut self) -> CH1_CMP_LOW_DATA_W<GP_CH1_CMP_DATA_SPEC> {
        CH1_CMP_LOW_DATA_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - Channel 1 Voltage High Value"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_cmp_hig_data(&mut self) -> CH1_CMP_HIG_DATA_W<GP_CH1_CMP_DATA_SPEC> {
        CH1_CMP_HIG_DATA_W::new(self, 16)
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
#[doc = "GPADC CH1 Compare Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gp_ch1_cmp_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gp_ch1_cmp_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GP_CH1_CMP_DATA_SPEC;
impl crate::RegisterSpec for GP_CH1_CMP_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gp_ch1_cmp_data::R`](R) reader structure"]
impl crate::Readable for GP_CH1_CMP_DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gp_ch1_cmp_data::W`](W) writer structure"]
impl crate::Writable for GP_CH1_CMP_DATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gp_ch1_cmp_data to value 0x0bff_0400"]
impl crate::Resettable for GP_CH1_CMP_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0x0bff_0400;
}
