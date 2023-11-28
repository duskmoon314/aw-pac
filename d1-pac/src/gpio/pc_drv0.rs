#[doc = "Register `pc_drv0` reader"]
pub type R = crate::R<PC_DRV0_SPEC>;
#[doc = "Register `pc_drv0` writer"]
pub type W = crate::W<PC_DRV0_SPEC>;
#[doc = "Field `pc_drv[0-7]` reader - PC Multi_Driving Select"]
pub type PC_DRV_R = crate::FieldReader<PC_DRV_A>;
#[doc = "PC Multi_Driving Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PC_DRV_A {
    #[doc = "0: `0`"]
    L0 = 0,
    #[doc = "1: `1`"]
    L1 = 1,
    #[doc = "2: `10`"]
    L2 = 2,
    #[doc = "3: `11`"]
    L3 = 3,
}
impl From<PC_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: PC_DRV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PC_DRV_A {
    type Ux = u8;
}
impl PC_DRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PC_DRV_A {
        match self.bits {
            0 => PC_DRV_A::L0,
            1 => PC_DRV_A::L1,
            2 => PC_DRV_A::L2,
            3 => PC_DRV_A::L3,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_l0(&self) -> bool {
        *self == PC_DRV_A::L0
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_l1(&self) -> bool {
        *self == PC_DRV_A::L1
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_l2(&self) -> bool {
        *self == PC_DRV_A::L2
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_l3(&self) -> bool {
        *self == PC_DRV_A::L3
    }
}
#[doc = "Field `pc_drv[0-7]` writer - PC Multi_Driving Select"]
pub type PC_DRV_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PC_DRV_A>;
impl<'a, REG> PC_DRV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn l0(self) -> &'a mut crate::W<REG> {
        self.variant(PC_DRV_A::L0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn l1(self) -> &'a mut crate::W<REG> {
        self.variant(PC_DRV_A::L1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn l2(self) -> &'a mut crate::W<REG> {
        self.variant(PC_DRV_A::L2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn l3(self) -> &'a mut crate::W<REG> {
        self.variant(PC_DRV_A::L3)
    }
}
impl R {
    #[doc = "PC Multi_Driving Select\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `pc0_drv` field"]
    #[inline(always)]
    pub fn pc_drv(&self, n: u8) -> PC_DRV_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        PC_DRV_R::new(((self.bits >> (n * 4)) & 3) as u8)
    }
    #[doc = "Bits 0:1 - PC Multi_Driving Select"]
    #[inline(always)]
    pub fn pc0_drv(&self) -> PC_DRV_R {
        PC_DRV_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - PC Multi_Driving Select"]
    #[inline(always)]
    pub fn pc1_drv(&self) -> PC_DRV_R {
        PC_DRV_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - PC Multi_Driving Select"]
    #[inline(always)]
    pub fn pc2_drv(&self) -> PC_DRV_R {
        PC_DRV_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PC Multi_Driving Select"]
    #[inline(always)]
    pub fn pc3_drv(&self) -> PC_DRV_R {
        PC_DRV_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - PC Multi_Driving Select"]
    #[inline(always)]
    pub fn pc4_drv(&self) -> PC_DRV_R {
        PC_DRV_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - PC Multi_Driving Select"]
    #[inline(always)]
    pub fn pc5_drv(&self) -> PC_DRV_R {
        PC_DRV_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - PC Multi_Driving Select"]
    #[inline(always)]
    pub fn pc6_drv(&self) -> PC_DRV_R {
        PC_DRV_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - PC Multi_Driving Select"]
    #[inline(always)]
    pub fn pc7_drv(&self) -> PC_DRV_R {
        PC_DRV_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "PC Multi_Driving Select\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `pc0_drv` field"]
    #[inline(always)]
    #[must_use]
    pub fn pc_drv(&mut self, n: u8) -> PC_DRV_W<PC_DRV0_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        PC_DRV_W::new(self, n * 4)
    }
    #[doc = "Bits 0:1 - PC Multi_Driving Select"]
    #[inline(always)]
    #[must_use]
    pub fn pc0_drv(&mut self) -> PC_DRV_W<PC_DRV0_SPEC> {
        PC_DRV_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - PC Multi_Driving Select"]
    #[inline(always)]
    #[must_use]
    pub fn pc1_drv(&mut self) -> PC_DRV_W<PC_DRV0_SPEC> {
        PC_DRV_W::new(self, 4)
    }
    #[doc = "Bits 8:9 - PC Multi_Driving Select"]
    #[inline(always)]
    #[must_use]
    pub fn pc2_drv(&mut self) -> PC_DRV_W<PC_DRV0_SPEC> {
        PC_DRV_W::new(self, 8)
    }
    #[doc = "Bits 12:13 - PC Multi_Driving Select"]
    #[inline(always)]
    #[must_use]
    pub fn pc3_drv(&mut self) -> PC_DRV_W<PC_DRV0_SPEC> {
        PC_DRV_W::new(self, 12)
    }
    #[doc = "Bits 16:17 - PC Multi_Driving Select"]
    #[inline(always)]
    #[must_use]
    pub fn pc4_drv(&mut self) -> PC_DRV_W<PC_DRV0_SPEC> {
        PC_DRV_W::new(self, 16)
    }
    #[doc = "Bits 20:21 - PC Multi_Driving Select"]
    #[inline(always)]
    #[must_use]
    pub fn pc5_drv(&mut self) -> PC_DRV_W<PC_DRV0_SPEC> {
        PC_DRV_W::new(self, 20)
    }
    #[doc = "Bits 24:25 - PC Multi_Driving Select"]
    #[inline(always)]
    #[must_use]
    pub fn pc6_drv(&mut self) -> PC_DRV_W<PC_DRV0_SPEC> {
        PC_DRV_W::new(self, 24)
    }
    #[doc = "Bits 28:29 - PC Multi_Driving Select"]
    #[inline(always)]
    #[must_use]
    pub fn pc7_drv(&mut self) -> PC_DRV_W<PC_DRV0_SPEC> {
        PC_DRV_W::new(self, 28)
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
#[doc = "PC Multi_Driving Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc_drv0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc_drv0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PC_DRV0_SPEC;
impl crate::RegisterSpec for PC_DRV0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pc_drv0::R`](R) reader structure"]
impl crate::Readable for PC_DRV0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pc_drv0::W`](W) writer structure"]
impl crate::Writable for PC_DRV0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pc_drv0 to value 0"]
impl crate::Resettable for PC_DRV0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
