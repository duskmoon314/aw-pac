#[doc = "Register `ic_chara` reader"]
pub type R = crate::R<IC_CHARA_SPEC>;
#[doc = "Register `ic_chara` writer"]
pub type W = crate::W<IC_CHARA_SPEC>;
#[doc = "Field `id_data` reader - Return 0x16AA only if the KEY_FIELD is set as 0x16AA when read those bits, otherwise return 0x0."]
pub type ID_DATA_R = crate::FieldReader<u16>;
#[doc = "Field `id_data` writer - Return 0x16AA only if the KEY_FIELD is set as 0x16AA when read those bits, otherwise return 0x0."]
pub type ID_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `key_field` reader - Key Field\n\nThe field should be written as 0x16AA. Writing any other value in this field aborts the write-operation."]
pub type KEY_FIELD_R = crate::FieldReader<u16>;
#[doc = "Field `key_field` writer - Key Field\n\nThe field should be written as 0x16AA. Writing any other value in this field aborts the write-operation."]
pub type KEY_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Return 0x16AA only if the KEY_FIELD is set as 0x16AA when read those bits, otherwise return 0x0."]
    #[inline(always)]
    pub fn id_data(&self) -> ID_DATA_R {
        ID_DATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Key Field\n\nThe field should be written as 0x16AA. Writing any other value in this field aborts the write-operation."]
    #[inline(always)]
    pub fn key_field(&self) -> KEY_FIELD_R {
        KEY_FIELD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Return 0x16AA only if the KEY_FIELD is set as 0x16AA when read those bits, otherwise return 0x0."]
    #[inline(always)]
    #[must_use]
    pub fn id_data(&mut self) -> ID_DATA_W<IC_CHARA_SPEC> {
        ID_DATA_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Key Field\n\nThe field should be written as 0x16AA. Writing any other value in this field aborts the write-operation."]
    #[inline(always)]
    #[must_use]
    pub fn key_field(&mut self) -> KEY_FIELD_W<IC_CHARA_SPEC> {
        KEY_FIELD_W::new(self, 16)
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
#[doc = "IC Characteristic Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ic_chara::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ic_chara::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IC_CHARA_SPEC;
impl crate::RegisterSpec for IC_CHARA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_chara::R`](R) reader structure"]
impl crate::Readable for IC_CHARA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ic_chara::W`](W) writer structure"]
impl crate::Writable for IC_CHARA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ic_chara to value 0"]
impl crate::Resettable for IC_CHARA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
