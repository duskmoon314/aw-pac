#[doc = "Register `tve_white_level` reader"]
pub type R = crate::R<TVE_WHITE_LEVEL_SPEC>;
#[doc = "Register `tve_white_level` writer"]
pub type W = crate::W<TVE_WHITE_LEVEL_SPEC>;
#[doc = "Field `white_level` reader - Specify the white level setting. 10-bit unsigned integer. Allowed range is from black_level+1 or vbi_blank_level +1 (whichever is greater) to 1023."]
pub type WHITE_LEVEL_R = crate::FieldReader<u16>;
#[doc = "Field `white_level` writer - Specify the white level setting. 10-bit unsigned integer. Allowed range is from black_level+1 or vbi_blank_level +1 (whichever is greater) to 1023."]
pub type WHITE_LEVEL_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `hd_sync_breezeway_level` reader - Specify the breezeway level setting. 10-bit unsigned integer. Allowed range is from 0 to 1023."]
pub type HD_SYNC_BREEZEWAY_LEVEL_R = crate::FieldReader<u16>;
#[doc = "Field `hd_sync_breezeway_level` writer - Specify the breezeway level setting. 10-bit unsigned integer. Allowed range is from 0 to 1023."]
pub type HD_SYNC_BREEZEWAY_LEVEL_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Specify the white level setting. 10-bit unsigned integer. Allowed range is from black_level+1 or vbi_blank_level +1 (whichever is greater) to 1023."]
    #[inline(always)]
    pub fn white_level(&self) -> WHITE_LEVEL_R {
        WHITE_LEVEL_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Specify the breezeway level setting. 10-bit unsigned integer. Allowed range is from 0 to 1023."]
    #[inline(always)]
    pub fn hd_sync_breezeway_level(&self) -> HD_SYNC_BREEZEWAY_LEVEL_R {
        HD_SYNC_BREEZEWAY_LEVEL_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Specify the white level setting. 10-bit unsigned integer. Allowed range is from black_level+1 or vbi_blank_level +1 (whichever is greater) to 1023."]
    #[inline(always)]
    #[must_use]
    pub fn white_level(&mut self) -> WHITE_LEVEL_W<TVE_WHITE_LEVEL_SPEC> {
        WHITE_LEVEL_W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Specify the breezeway level setting. 10-bit unsigned integer. Allowed range is from 0 to 1023."]
    #[inline(always)]
    #[must_use]
    pub fn hd_sync_breezeway_level(&mut self) -> HD_SYNC_BREEZEWAY_LEVEL_W<TVE_WHITE_LEVEL_SPEC> {
        HD_SYNC_BREEZEWAY_LEVEL_W::new(self, 16)
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
#[doc = "TV Encoder White Level Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_white_level::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_white_level::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TVE_WHITE_LEVEL_SPEC;
impl crate::RegisterSpec for TVE_WHITE_LEVEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tve_white_level::R`](R) reader structure"]
impl crate::Readable for TVE_WHITE_LEVEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tve_white_level::W`](W) writer structure"]
impl crate::Writable for TVE_WHITE_LEVEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_white_level to value 0x01e8_0320"]
impl crate::Resettable for TVE_WHITE_LEVEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x01e8_0320;
}
