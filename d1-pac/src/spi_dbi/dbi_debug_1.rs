#[doc = "Register `dbi_debug_1` reader"]
pub type R = crate::R<DBI_DEBUG_1_SPEC>;
#[doc = "Field `ccnt` reader - "]
pub type CCNT_R = crate::FieldReader<u16>;
#[doc = "Field `lcnt` reader - "]
pub type LCNT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn ccnt(&self) -> CCNT_R {
        CCNT_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn lcnt(&self) -> LCNT_R {
        LCNT_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[doc = "DBI BEBUG 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbi_debug_1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBI_DEBUG_1_SPEC;
impl crate::RegisterSpec for DBI_DEBUG_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbi_debug_1::R`](R) reader structure"]
impl crate::Readable for DBI_DEBUG_1_SPEC {}
#[doc = "`reset()` method sets dbi_debug_1 to value 0"]
impl crate::Resettable for DBI_DEBUG_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
