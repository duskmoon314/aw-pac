#[doc = "Register `alarm0_cur_vlu_reg` reader"]
pub struct R(crate::R<ALARM0_CUR_VLU_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALARM0_CUR_VLU_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALARM0_CUR_VLU_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALARM0_CUR_VLU_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `alarm0_cur_vlu_reg` writer"]
pub struct W(crate::W<ALARM0_CUR_VLU_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALARM0_CUR_VLU_REG_SPEC>;
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
impl From<crate::W<ALARM0_CUR_VLU_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALARM0_CUR_VLU_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `hour` reader - Current hour Range from 0 to 23."]
pub type HOUR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `hour` writer - Current hour Range from 0 to 23."]
pub type HOUR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ALARM0_CUR_VLU_REG_SPEC, u8, u8, 5, O>;
#[doc = "Field `minute` reader - Current minute Range from 0 to 59."]
pub type MINUTE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `minute` writer - Current minute Range from 0 to 59."]
pub type MINUTE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ALARM0_CUR_VLU_REG_SPEC, u8, u8, 6, O>;
#[doc = "Field `second` reader - Current second Range from 0 to 59."]
pub type SECOND_R = crate::FieldReader<u8, u8>;
#[doc = "Field `second` writer - Current second Range from 0 to 59."]
pub type SECOND_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ALARM0_CUR_VLU_REG_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 16:20 - Current hour Range from 0 to 23."]
    #[inline(always)]
    pub fn hour(&self) -> HOUR_R {
        HOUR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 8:13 - Current minute Range from 0 to 59."]
    #[inline(always)]
    pub fn minute(&self) -> MINUTE_R {
        MINUTE_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5 - Current second Range from 0 to 59."]
    #[inline(always)]
    pub fn second(&self) -> SECOND_R {
        SECOND_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:20 - Current hour Range from 0 to 23."]
    #[inline(always)]
    pub fn hour(&mut self) -> HOUR_W<16> {
        HOUR_W::new(self)
    }
    #[doc = "Bits 8:13 - Current minute Range from 0 to 59."]
    #[inline(always)]
    pub fn minute(&mut self) -> MINUTE_W<8> {
        MINUTE_W::new(self)
    }
    #[doc = "Bits 0:5 - Current second Range from 0 to 59."]
    #[inline(always)]
    pub fn second(&mut self) -> SECOND_W<0> {
        SECOND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alarm 0 Counter Current Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alarm0_cur_vlu_reg](index.html) module"]
pub struct ALARM0_CUR_VLU_REG_SPEC;
impl crate::RegisterSpec for ALARM0_CUR_VLU_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alarm0_cur_vlu_reg::R](R) reader structure"]
impl crate::Readable for ALARM0_CUR_VLU_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [alarm0_cur_vlu_reg::W](W) writer structure"]
impl crate::Writable for ALARM0_CUR_VLU_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets alarm0_cur_vlu_reg to value 0"]
impl crate::Resettable for ALARM0_CUR_VLU_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
