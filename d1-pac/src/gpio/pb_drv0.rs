#[doc = "Register `pb_drv0` reader"]
pub type R = crate::R<PB_DRV0_SPEC>;
#[doc = "Register `pb_drv0` writer"]
pub type W = crate::W<PB_DRV0_SPEC>;
#[doc = "Field `pb_drv[0-7]` reader - PB Multi_Driving Select"]
pub type PB_DRV_R = crate::FieldReader<PB_DRV_A>;
#[doc = "PB Multi_Driving Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PB_DRV_A {
    #[doc = "0: `0`"]
    L0 = 0,
    #[doc = "1: `1`"]
    L1 = 1,
    #[doc = "2: `10`"]
    L2 = 2,
    #[doc = "3: `11`"]
    L3 = 3,
}
impl From<PB_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: PB_DRV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PB_DRV_A {
    type Ux = u8;
}
impl PB_DRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PB_DRV_A {
        match self.bits {
            0 => PB_DRV_A::L0,
            1 => PB_DRV_A::L1,
            2 => PB_DRV_A::L2,
            3 => PB_DRV_A::L3,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_l0(&self) -> bool {
        *self == PB_DRV_A::L0
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_l1(&self) -> bool {
        *self == PB_DRV_A::L1
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_l2(&self) -> bool {
        *self == PB_DRV_A::L2
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_l3(&self) -> bool {
        *self == PB_DRV_A::L3
    }
}
#[doc = "Field `pb_drv[0-7]` writer - PB Multi_Driving Select"]
pub type PB_DRV_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PB_DRV_A>;
impl<'a, REG> PB_DRV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn l0(self) -> &'a mut crate::W<REG> {
        self.variant(PB_DRV_A::L0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn l1(self) -> &'a mut crate::W<REG> {
        self.variant(PB_DRV_A::L1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn l2(self) -> &'a mut crate::W<REG> {
        self.variant(PB_DRV_A::L2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn l3(self) -> &'a mut crate::W<REG> {
        self.variant(PB_DRV_A::L3)
    }
}
impl R {
    #[doc = "PB Multi_Driving Select\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `pb0_drv` field"]
    #[inline(always)]
    pub fn pb_drv(&self, n: u8) -> PB_DRV_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        PB_DRV_R::new(((self.bits >> (n * 4)) & 3) as u8)
    }
    #[doc = "Bits 0:1 - PB Multi_Driving Select"]
    #[inline(always)]
    pub fn pb0_drv(&self) -> PB_DRV_R {
        PB_DRV_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - PB Multi_Driving Select"]
    #[inline(always)]
    pub fn pb1_drv(&self) -> PB_DRV_R {
        PB_DRV_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - PB Multi_Driving Select"]
    #[inline(always)]
    pub fn pb2_drv(&self) -> PB_DRV_R {
        PB_DRV_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PB Multi_Driving Select"]
    #[inline(always)]
    pub fn pb3_drv(&self) -> PB_DRV_R {
        PB_DRV_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - PB Multi_Driving Select"]
    #[inline(always)]
    pub fn pb4_drv(&self) -> PB_DRV_R {
        PB_DRV_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - PB Multi_Driving Select"]
    #[inline(always)]
    pub fn pb5_drv(&self) -> PB_DRV_R {
        PB_DRV_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - PB Multi_Driving Select"]
    #[inline(always)]
    pub fn pb6_drv(&self) -> PB_DRV_R {
        PB_DRV_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - PB Multi_Driving Select"]
    #[inline(always)]
    pub fn pb7_drv(&self) -> PB_DRV_R {
        PB_DRV_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "PB Multi_Driving Select\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `pb0_drv` field"]
    #[inline(always)]
    #[must_use]
    pub fn pb_drv(&mut self, n: u8) -> PB_DRV_W<PB_DRV0_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        PB_DRV_W::new(self, n * 4)
    }
    #[doc = "Bits 0:1 - PB Multi_Driving Select"]
    #[inline(always)]
    #[must_use]
    pub fn pb0_drv(&mut self) -> PB_DRV_W<PB_DRV0_SPEC> {
        PB_DRV_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - PB Multi_Driving Select"]
    #[inline(always)]
    #[must_use]
    pub fn pb1_drv(&mut self) -> PB_DRV_W<PB_DRV0_SPEC> {
        PB_DRV_W::new(self, 4)
    }
    #[doc = "Bits 8:9 - PB Multi_Driving Select"]
    #[inline(always)]
    #[must_use]
    pub fn pb2_drv(&mut self) -> PB_DRV_W<PB_DRV0_SPEC> {
        PB_DRV_W::new(self, 8)
    }
    #[doc = "Bits 12:13 - PB Multi_Driving Select"]
    #[inline(always)]
    #[must_use]
    pub fn pb3_drv(&mut self) -> PB_DRV_W<PB_DRV0_SPEC> {
        PB_DRV_W::new(self, 12)
    }
    #[doc = "Bits 16:17 - PB Multi_Driving Select"]
    #[inline(always)]
    #[must_use]
    pub fn pb4_drv(&mut self) -> PB_DRV_W<PB_DRV0_SPEC> {
        PB_DRV_W::new(self, 16)
    }
    #[doc = "Bits 20:21 - PB Multi_Driving Select"]
    #[inline(always)]
    #[must_use]
    pub fn pb5_drv(&mut self) -> PB_DRV_W<PB_DRV0_SPEC> {
        PB_DRV_W::new(self, 20)
    }
    #[doc = "Bits 24:25 - PB Multi_Driving Select"]
    #[inline(always)]
    #[must_use]
    pub fn pb6_drv(&mut self) -> PB_DRV_W<PB_DRV0_SPEC> {
        PB_DRV_W::new(self, 24)
    }
    #[doc = "Bits 28:29 - PB Multi_Driving Select"]
    #[inline(always)]
    #[must_use]
    pub fn pb7_drv(&mut self) -> PB_DRV_W<PB_DRV0_SPEC> {
        PB_DRV_W::new(self, 28)
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
#[doc = "PB Multi_Driving Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb_drv0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb_drv0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PB_DRV0_SPEC;
impl crate::RegisterSpec for PB_DRV0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pb_drv0::R`](R) reader structure"]
impl crate::Readable for PB_DRV0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pb_drv0::W`](W) writer structure"]
impl crate::Writable for PB_DRV0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pb_drv0 to value 0"]
impl crate::Resettable for PB_DRV0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
