#[doc = "Register `iommu_bypass` reader"]
pub type R = crate::R<IOMMU_BYPASS_SPEC>;
#[doc = "Register `iommu_bypass` writer"]
pub type W = crate::W<IOMMU_BYPASS_SPEC>;
#[doc = "Field `m_bp[0-6]` reader - Master\\[i\\] bypass switch\n\nAfter bypass function is opened, IOMMU can not map the address of Master6 sending, and directly output the virtual address to MBUS as physical address."]
pub type M_BP_R = crate::BitReader<M_BP_A>;
#[doc = "Master\\[i\\] bypass switch\n\nAfter bypass function is opened, IOMMU can not map the address of Master6 sending, and directly output the virtual address to MBUS as physical address.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M_BP_A {
    #[doc = "0: Disable bypass function"]
    DISABLE = 0,
    #[doc = "1: Enable bypass function"]
    ENABLE = 1,
}
impl From<M_BP_A> for bool {
    #[inline(always)]
    fn from(variant: M_BP_A) -> Self {
        variant as u8 != 0
    }
}
impl M_BP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> M_BP_A {
        match self.bits {
            false => M_BP_A::DISABLE,
            true => M_BP_A::ENABLE,
        }
    }
    #[doc = "Disable bypass function"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == M_BP_A::DISABLE
    }
    #[doc = "Enable bypass function"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == M_BP_A::ENABLE
    }
}
#[doc = "Field `m_bp[0-6]` writer - Master\\[i\\] bypass switch\n\nAfter bypass function is opened, IOMMU can not map the address of Master6 sending, and directly output the virtual address to MBUS as physical address."]
pub type M_BP_W<'a, REG> = crate::BitWriter<'a, REG, M_BP_A>;
impl<'a, REG> M_BP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable bypass function"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(M_BP_A::DISABLE)
    }
    #[doc = "Enable bypass function"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(M_BP_A::ENABLE)
    }
}
impl R {
    #[doc = "Master\\[i\\] bypass switch\n\nAfter bypass function is opened, IOMMU can not map the address of Master6 sending, and directly output the virtual address to MBUS as physical address.\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `m0_bp` field"]
    #[inline(always)]
    pub fn m_bp(&self, n: u8) -> M_BP_R {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        M_BP_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - Master\\[i\\] bypass switch\n\nAfter bypass function is opened, IOMMU can not map the address of Master6 sending, and directly output the virtual address to MBUS as physical address."]
    #[inline(always)]
    pub fn m0_bp(&self) -> M_BP_R {
        M_BP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Master\\[i\\] bypass switch\n\nAfter bypass function is opened, IOMMU can not map the address of Master6 sending, and directly output the virtual address to MBUS as physical address."]
    #[inline(always)]
    pub fn m1_bp(&self) -> M_BP_R {
        M_BP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master\\[i\\] bypass switch\n\nAfter bypass function is opened, IOMMU can not map the address of Master6 sending, and directly output the virtual address to MBUS as physical address."]
    #[inline(always)]
    pub fn m2_bp(&self) -> M_BP_R {
        M_BP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Master\\[i\\] bypass switch\n\nAfter bypass function is opened, IOMMU can not map the address of Master6 sending, and directly output the virtual address to MBUS as physical address."]
    #[inline(always)]
    pub fn m3_bp(&self) -> M_BP_R {
        M_BP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Master\\[i\\] bypass switch\n\nAfter bypass function is opened, IOMMU can not map the address of Master6 sending, and directly output the virtual address to MBUS as physical address."]
    #[inline(always)]
    pub fn m4_bp(&self) -> M_BP_R {
        M_BP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Master\\[i\\] bypass switch\n\nAfter bypass function is opened, IOMMU can not map the address of Master6 sending, and directly output the virtual address to MBUS as physical address."]
    #[inline(always)]
    pub fn m5_bp(&self) -> M_BP_R {
        M_BP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Master\\[i\\] bypass switch\n\nAfter bypass function is opened, IOMMU can not map the address of Master6 sending, and directly output the virtual address to MBUS as physical address."]
    #[inline(always)]
    pub fn m6_bp(&self) -> M_BP_R {
        M_BP_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Master\\[i\\] bypass switch\n\nAfter bypass function is opened, IOMMU can not map the address of Master6 sending, and directly output the virtual address to MBUS as physical address.\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `m0_bp` field"]
    #[inline(always)]
    #[must_use]
    pub fn m_bp(&mut self, n: u8) -> M_BP_W<IOMMU_BYPASS_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        M_BP_W::new(self, n)
    }
    #[doc = "Bit 0 - Master\\[i\\] bypass switch\n\nAfter bypass function is opened, IOMMU can not map the address of Master6 sending, and directly output the virtual address to MBUS as physical address."]
    #[inline(always)]
    #[must_use]
    pub fn m0_bp(&mut self) -> M_BP_W<IOMMU_BYPASS_SPEC> {
        M_BP_W::new(self, 0)
    }
    #[doc = "Bit 1 - Master\\[i\\] bypass switch\n\nAfter bypass function is opened, IOMMU can not map the address of Master6 sending, and directly output the virtual address to MBUS as physical address."]
    #[inline(always)]
    #[must_use]
    pub fn m1_bp(&mut self) -> M_BP_W<IOMMU_BYPASS_SPEC> {
        M_BP_W::new(self, 1)
    }
    #[doc = "Bit 2 - Master\\[i\\] bypass switch\n\nAfter bypass function is opened, IOMMU can not map the address of Master6 sending, and directly output the virtual address to MBUS as physical address."]
    #[inline(always)]
    #[must_use]
    pub fn m2_bp(&mut self) -> M_BP_W<IOMMU_BYPASS_SPEC> {
        M_BP_W::new(self, 2)
    }
    #[doc = "Bit 3 - Master\\[i\\] bypass switch\n\nAfter bypass function is opened, IOMMU can not map the address of Master6 sending, and directly output the virtual address to MBUS as physical address."]
    #[inline(always)]
    #[must_use]
    pub fn m3_bp(&mut self) -> M_BP_W<IOMMU_BYPASS_SPEC> {
        M_BP_W::new(self, 3)
    }
    #[doc = "Bit 4 - Master\\[i\\] bypass switch\n\nAfter bypass function is opened, IOMMU can not map the address of Master6 sending, and directly output the virtual address to MBUS as physical address."]
    #[inline(always)]
    #[must_use]
    pub fn m4_bp(&mut self) -> M_BP_W<IOMMU_BYPASS_SPEC> {
        M_BP_W::new(self, 4)
    }
    #[doc = "Bit 5 - Master\\[i\\] bypass switch\n\nAfter bypass function is opened, IOMMU can not map the address of Master6 sending, and directly output the virtual address to MBUS as physical address."]
    #[inline(always)]
    #[must_use]
    pub fn m5_bp(&mut self) -> M_BP_W<IOMMU_BYPASS_SPEC> {
        M_BP_W::new(self, 5)
    }
    #[doc = "Bit 6 - Master\\[i\\] bypass switch\n\nAfter bypass function is opened, IOMMU can not map the address of Master6 sending, and directly output the virtual address to MBUS as physical address."]
    #[inline(always)]
    #[must_use]
    pub fn m6_bp(&mut self) -> M_BP_W<IOMMU_BYPASS_SPEC> {
        M_BP_W::new(self, 6)
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
#[doc = "IOMMU Bypass Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_bypass::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_bypass::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOMMU_BYPASS_SPEC;
impl crate::RegisterSpec for IOMMU_BYPASS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iommu_bypass::R`](R) reader structure"]
impl crate::Readable for IOMMU_BYPASS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iommu_bypass::W`](W) writer structure"]
impl crate::Writable for IOMMU_BYPASS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets iommu_bypass to value 0x7f"]
impl crate::Resettable for IOMMU_BYPASS_SPEC {
    const RESET_VALUE: Self::Ux = 0x7f;
}
