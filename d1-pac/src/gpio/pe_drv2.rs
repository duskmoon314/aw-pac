#[doc = "Register `pe_drv2` reader"]
pub type R = crate::R<PE_DRV2_SPEC>;
#[doc = "Register `pe_drv2` writer"]
pub type W = crate::W<PE_DRV2_SPEC>;
#[doc = "Field `pe_drv[16-17]` reader - PE Multi_Driving Select"]
pub type PE_DRV_R = crate::FieldReader<PE_DRV_A>;
#[doc = "PE Multi_Driving Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PE_DRV_A {
    #[doc = "0: `0`"]
    L0 = 0,
    #[doc = "1: `1`"]
    L1 = 1,
    #[doc = "2: `10`"]
    L2 = 2,
    #[doc = "3: `11`"]
    L3 = 3,
}
impl From<PE_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: PE_DRV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PE_DRV_A {
    type Ux = u8;
}
impl PE_DRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PE_DRV_A {
        match self.bits {
            0 => PE_DRV_A::L0,
            1 => PE_DRV_A::L1,
            2 => PE_DRV_A::L2,
            3 => PE_DRV_A::L3,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_l0(&self) -> bool {
        *self == PE_DRV_A::L0
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_l1(&self) -> bool {
        *self == PE_DRV_A::L1
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_l2(&self) -> bool {
        *self == PE_DRV_A::L2
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_l3(&self) -> bool {
        *self == PE_DRV_A::L3
    }
}
#[doc = "Field `pe_drv[16-17]` writer - PE Multi_Driving Select"]
pub type PE_DRV_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PE_DRV_A>;
impl<'a, REG> PE_DRV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn l0(self) -> &'a mut crate::W<REG> {
        self.variant(PE_DRV_A::L0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn l1(self) -> &'a mut crate::W<REG> {
        self.variant(PE_DRV_A::L1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn l2(self) -> &'a mut crate::W<REG> {
        self.variant(PE_DRV_A::L2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn l3(self) -> &'a mut crate::W<REG> {
        self.variant(PE_DRV_A::L3)
    }
}
impl R {
    #[doc = "PE Multi_Driving Select\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `pe16_drv` field"]
    #[inline(always)]
    pub fn pe_drv(&self, n: u8) -> PE_DRV_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        PE_DRV_R::new(((self.bits >> (n * 4)) & 3) as u8)
    }
    #[doc = "Bits 0:1 - PE Multi_Driving Select"]
    #[inline(always)]
    pub fn pe16_drv(&self) -> PE_DRV_R {
        PE_DRV_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - PE Multi_Driving Select"]
    #[inline(always)]
    pub fn pe17_drv(&self) -> PE_DRV_R {
        PE_DRV_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "PE Multi_Driving Select\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `pe16_drv` field"]
    #[inline(always)]
    #[must_use]
    pub fn pe_drv(&mut self, n: u8) -> PE_DRV_W<PE_DRV2_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        PE_DRV_W::new(self, n * 4)
    }
    #[doc = "Bits 0:1 - PE Multi_Driving Select"]
    #[inline(always)]
    #[must_use]
    pub fn pe16_drv(&mut self) -> PE_DRV_W<PE_DRV2_SPEC> {
        PE_DRV_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - PE Multi_Driving Select"]
    #[inline(always)]
    #[must_use]
    pub fn pe17_drv(&mut self) -> PE_DRV_W<PE_DRV2_SPEC> {
        PE_DRV_W::new(self, 4)
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
#[doc = "PE Multi_Driving Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pe_drv2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pe_drv2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PE_DRV2_SPEC;
impl crate::RegisterSpec for PE_DRV2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pe_drv2::R`](R) reader structure"]
impl crate::Readable for PE_DRV2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pe_drv2::W`](W) writer structure"]
impl crate::Writable for PE_DRV2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pe_drv2 to value 0"]
impl crate::Resettable for PE_DRV2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
