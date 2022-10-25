#[doc = "Register `ic_chara` reader"]
pub struct R(crate::R<IC_CHARA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IC_CHARA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IC_CHARA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IC_CHARA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ic_chara` writer"]
pub struct W(crate::W<IC_CHARA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IC_CHARA_SPEC>;
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
impl From<crate::W<IC_CHARA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IC_CHARA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `id_data` reader - Return 0x16AA only if the KEY_FIELD is set as 0x16AA when read those bits, otherwise return 0x0."]
pub type ID_DATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `id_data` writer - Return 0x16AA only if the KEY_FIELD is set as 0x16AA when read those bits, otherwise return 0x0."]
pub type ID_DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IC_CHARA_SPEC, u16, u16, 16, O>;
#[doc = "Field `key_field` reader - Key Field\n\nThe field should be written as 0x16AA. Writing any other value in this field aborts the write-operation."]
pub type KEY_FIELD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `key_field` writer - Key Field\n\nThe field should be written as 0x16AA. Writing any other value in this field aborts the write-operation."]
pub type KEY_FIELD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IC_CHARA_SPEC, u16, u16, 16, O>;
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
    pub fn id_data(&mut self) -> ID_DATA_W<0> {
        ID_DATA_W::new(self)
    }
    #[doc = "Bits 16:31 - Key Field\n\nThe field should be written as 0x16AA. Writing any other value in this field aborts the write-operation."]
    #[inline(always)]
    #[must_use]
    pub fn key_field(&mut self) -> KEY_FIELD_W<16> {
        KEY_FIELD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IC Characteristic Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic_chara](index.html) module"]
pub struct IC_CHARA_SPEC;
impl crate::RegisterSpec for IC_CHARA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ic_chara::R](R) reader structure"]
impl crate::Readable for IC_CHARA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ic_chara::W](W) writer structure"]
impl crate::Writable for IC_CHARA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ic_chara to value 0"]
impl crate::Resettable for IC_CHARA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
