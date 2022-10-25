#[doc = "Register `lcd_cpu_tri1` reader"]
pub struct R(crate::R<LCD_CPU_TRI1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_CPU_TRI1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_CPU_TRI1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_CPU_TRI1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lcd_cpu_tri1` writer"]
pub struct W(crate::W<LCD_CPU_TRI1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_CPU_TRI1_SPEC>;
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
impl From<crate::W<LCD_CPU_TRI1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_CPU_TRI1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `block_num` reader - The number of data blocks. It is usually set as Y."]
pub type BLOCK_NUM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `block_num` writer - The number of data blocks. It is usually set as Y."]
pub type BLOCK_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LCD_CPU_TRI1_SPEC, u16, u16, 16, O>;
#[doc = "Field `block_current_num` reader - Shows the current data block transmitting to panel."]
pub type BLOCK_CURRENT_NUM_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - The number of data blocks. It is usually set as Y."]
    #[inline(always)]
    pub fn block_num(&self) -> BLOCK_NUM_R {
        BLOCK_NUM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Shows the current data block transmitting to panel."]
    #[inline(always)]
    pub fn block_current_num(&self) -> BLOCK_CURRENT_NUM_R {
        BLOCK_CURRENT_NUM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The number of data blocks. It is usually set as Y."]
    #[inline(always)]
    #[must_use]
    pub fn block_num(&mut self) -> BLOCK_NUM_W<0> {
        BLOCK_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD CPU Panel Trigger Register1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_cpu_tri1](index.html) module"]
pub struct LCD_CPU_TRI1_SPEC;
impl crate::RegisterSpec for LCD_CPU_TRI1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_cpu_tri1::R](R) reader structure"]
impl crate::Readable for LCD_CPU_TRI1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_cpu_tri1::W](W) writer structure"]
impl crate::Writable for LCD_CPU_TRI1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_cpu_tri1 to value 0"]
impl crate::Resettable for LCD_CPU_TRI1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
