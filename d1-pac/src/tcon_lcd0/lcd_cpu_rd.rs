#[doc = "Register `lcd_cpu_rd%s` reader"]
pub struct R(crate::R<LCD_CPU_RD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_CPU_RD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_CPU_RD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_CPU_RD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lcd_cpu_rd%s` writer"]
pub struct W(crate::W<LCD_CPU_RD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_CPU_RD_SPEC>;
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
impl From<crate::W<LCD_CPU_RD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_CPU_RD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `data_rd0` reader - Data read on 8080 bus, launch a new read operation on 8080 bus."]
pub type DATA_RD0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - Data read on 8080 bus, launch a new read operation on 8080 bus."]
    #[inline(always)]
    pub fn data_rd0(&self) -> DATA_RD0_R {
        DATA_RD0_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD CPU Panel Read Data Register\\[i\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_cpu_rd](index.html) module"]
pub struct LCD_CPU_RD_SPEC;
impl crate::RegisterSpec for LCD_CPU_RD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_cpu_rd::R](R) reader structure"]
impl crate::Readable for LCD_CPU_RD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_cpu_rd::W](W) writer structure"]
impl crate::Writable for LCD_CPU_RD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_cpu_rd%s to value 0"]
impl crate::Resettable for LCD_CPU_RD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
