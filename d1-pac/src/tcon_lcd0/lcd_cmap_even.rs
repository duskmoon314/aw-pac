#[doc = "Register `lcd_cmap_even%s` reader"]
pub type R = crate::R<LCD_CMAP_EVEN_SPEC>;
#[doc = "Register `lcd_cmap_even%s` writer"]
pub type W = crate::W<LCD_CMAP_EVEN_SPEC>;
#[doc = "Field `out_even[0-1]` reader - OUT_EVEN\\[2i + j\\]\n\nIndicates the output order of components.\n\nbit15-12: Reserved\n\nbit11-08: Out_Even\\[23:16\\]\n\nbit07-04: Out_Even0\\[15:8\\]\n\nbit03-00: Out_Even0\\[7:0\\]"]
pub type OUT_EVEN_R = crate::FieldReader<u16>;
#[doc = "Field `out_even[0-1]` writer - OUT_EVEN\\[2i + j\\]\n\nIndicates the output order of components.\n\nbit15-12: Reserved\n\nbit11-08: Out_Even\\[23:16\\]\n\nbit07-04: Out_Even0\\[15:8\\]\n\nbit03-00: Out_Even0\\[7:0\\]"]
pub type OUT_EVEN_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "OUT_EVEN\\[2i + j\\]\n\nIndicates the output order of components.\n\nbit15-12: Reserved\n\nbit11-08: Out_Even\\[23:16\\]\n\nbit07-04: Out_Even0\\[15:8\\]\n\nbit03-00: Out_Even0\\[7:0\\]\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `out_even0` field"]
    #[inline(always)]
    pub fn out_even(&self, n: u8) -> OUT_EVEN_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OUT_EVEN_R::new(((self.bits >> (n * 16)) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - OUT_EVEN\\[2i + j\\]\n\nIndicates the output order of components.\n\nbit15-12: Reserved\n\nbit11-08: Out_Even\\[23:16\\]\n\nbit07-04: Out_Even0\\[15:8\\]\n\nbit03-00: Out_Even0\\[7:0\\]"]
    #[inline(always)]
    pub fn out_even0(&self) -> OUT_EVEN_R {
        OUT_EVEN_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - OUT_EVEN\\[2i + j\\]\n\nIndicates the output order of components.\n\nbit15-12: Reserved\n\nbit11-08: Out_Even\\[23:16\\]\n\nbit07-04: Out_Even0\\[15:8\\]\n\nbit03-00: Out_Even0\\[7:0\\]"]
    #[inline(always)]
    pub fn out_even1(&self) -> OUT_EVEN_R {
        OUT_EVEN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "OUT_EVEN\\[2i + j\\]\n\nIndicates the output order of components.\n\nbit15-12: Reserved\n\nbit11-08: Out_Even\\[23:16\\]\n\nbit07-04: Out_Even0\\[15:8\\]\n\nbit03-00: Out_Even0\\[7:0\\]\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `out_even0` field"]
    #[inline(always)]
    #[must_use]
    pub fn out_even(&mut self, n: u8) -> OUT_EVEN_W<LCD_CMAP_EVEN_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OUT_EVEN_W::new(self, n * 16)
    }
    #[doc = "Bits 0:15 - OUT_EVEN\\[2i + j\\]\n\nIndicates the output order of components.\n\nbit15-12: Reserved\n\nbit11-08: Out_Even\\[23:16\\]\n\nbit07-04: Out_Even0\\[15:8\\]\n\nbit03-00: Out_Even0\\[7:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn out_even0(&mut self) -> OUT_EVEN_W<LCD_CMAP_EVEN_SPEC> {
        OUT_EVEN_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - OUT_EVEN\\[2i + j\\]\n\nIndicates the output order of components.\n\nbit15-12: Reserved\n\nbit11-08: Out_Even\\[23:16\\]\n\nbit07-04: Out_Even0\\[15:8\\]\n\nbit03-00: Out_Even0\\[7:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn out_even1(&mut self) -> OUT_EVEN_W<LCD_CMAP_EVEN_SPEC> {
        OUT_EVEN_W::new(self, 16)
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
#[doc = "LCD Color Map Even Line Register\\[i\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_cmap_even::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_cmap_even::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_CMAP_EVEN_SPEC;
impl crate::RegisterSpec for LCD_CMAP_EVEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_cmap_even::R`](R) reader structure"]
impl crate::Readable for LCD_CMAP_EVEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_cmap_even::W`](W) writer structure"]
impl crate::Writable for LCD_CMAP_EVEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_cmap_even%s to value 0"]
impl crate::Resettable for LCD_CMAP_EVEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
