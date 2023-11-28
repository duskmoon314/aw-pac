#[doc = "Register `lcd_hv_if` reader"]
pub type R = crate::R<LCD_HV_IF_SPEC>;
#[doc = "Register `lcd_hv_if` writer"]
pub type W = crate::W<LCD_HV_IF_SPEC>;
#[doc = "Field `ccir_csc_dis` reader - LCD convert source from RGB to YUV.\n\nOnly valid when HV mode is “1100”."]
pub type CCIR_CSC_DIS_R = crate::BitReader<CCIR_CSC_DIS_A>;
#[doc = "LCD convert source from RGB to YUV.\n\nOnly valid when HV mode is “1100”.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCIR_CSC_DIS_A {
    #[doc = "0: Enable"]
    ENABLE = 0,
    #[doc = "1: Disable"]
    DISABLE = 1,
}
impl From<CCIR_CSC_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: CCIR_CSC_DIS_A) -> Self {
        variant as u8 != 0
    }
}
impl CCIR_CSC_DIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCIR_CSC_DIS_A {
        match self.bits {
            false => CCIR_CSC_DIS_A::ENABLE,
            true => CCIR_CSC_DIS_A::DISABLE,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CCIR_CSC_DIS_A::ENABLE
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CCIR_CSC_DIS_A::DISABLE
    }
}
#[doc = "Field `ccir_csc_dis` writer - LCD convert source from RGB to YUV.\n\nOnly valid when HV mode is “1100”."]
pub type CCIR_CSC_DIS_W<'a, REG> = crate::BitWriter<'a, REG, CCIR_CSC_DIS_A>;
impl<'a, REG> CCIR_CSC_DIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CCIR_CSC_DIS_A::ENABLE)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CCIR_CSC_DIS_A::DISABLE)
    }
}
#[doc = "Field `yuv_eav_sav_f_line_dly` reader - Set the delay line mode."]
pub type YUV_EAV_SAV_F_LINE_DLY_R = crate::FieldReader<YUV_EAV_SAV_F_LINE_DLY_A>;
#[doc = "Set the delay line mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum YUV_EAV_SAV_F_LINE_DLY_A {
    #[doc = "0: F toggle right after active video line"]
    F_TOGGLE = 0,
    #[doc = "1: delay 2 line (CCIR PAL)"]
    CCIR_PAL = 1,
    #[doc = "2: delay 3 line (CCIR NTSC)"]
    CCIR_NTSC = 2,
}
impl From<YUV_EAV_SAV_F_LINE_DLY_A> for u8 {
    #[inline(always)]
    fn from(variant: YUV_EAV_SAV_F_LINE_DLY_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for YUV_EAV_SAV_F_LINE_DLY_A {
    type Ux = u8;
}
impl YUV_EAV_SAV_F_LINE_DLY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<YUV_EAV_SAV_F_LINE_DLY_A> {
        match self.bits {
            0 => Some(YUV_EAV_SAV_F_LINE_DLY_A::F_TOGGLE),
            1 => Some(YUV_EAV_SAV_F_LINE_DLY_A::CCIR_PAL),
            2 => Some(YUV_EAV_SAV_F_LINE_DLY_A::CCIR_NTSC),
            _ => None,
        }
    }
    #[doc = "F toggle right after active video line"]
    #[inline(always)]
    pub fn is_f_toggle(&self) -> bool {
        *self == YUV_EAV_SAV_F_LINE_DLY_A::F_TOGGLE
    }
    #[doc = "delay 2 line (CCIR PAL)"]
    #[inline(always)]
    pub fn is_ccir_pal(&self) -> bool {
        *self == YUV_EAV_SAV_F_LINE_DLY_A::CCIR_PAL
    }
    #[doc = "delay 3 line (CCIR NTSC)"]
    #[inline(always)]
    pub fn is_ccir_ntsc(&self) -> bool {
        *self == YUV_EAV_SAV_F_LINE_DLY_A::CCIR_NTSC
    }
}
#[doc = "Field `yuv_eav_sav_f_line_dly` writer - Set the delay line mode."]
pub type YUV_EAV_SAV_F_LINE_DLY_W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, YUV_EAV_SAV_F_LINE_DLY_A>;
impl<'a, REG> YUV_EAV_SAV_F_LINE_DLY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "F toggle right after active video line"]
    #[inline(always)]
    pub fn f_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(YUV_EAV_SAV_F_LINE_DLY_A::F_TOGGLE)
    }
    #[doc = "delay 2 line (CCIR PAL)"]
    #[inline(always)]
    pub fn ccir_pal(self) -> &'a mut crate::W<REG> {
        self.variant(YUV_EAV_SAV_F_LINE_DLY_A::CCIR_PAL)
    }
    #[doc = "delay 3 line (CCIR NTSC)"]
    #[inline(always)]
    pub fn ccir_ntsc(self) -> &'a mut crate::W<REG> {
        self.variant(YUV_EAV_SAV_F_LINE_DLY_A::CCIR_NTSC)
    }
}
#[doc = "Field `yuv_sm` reader - Serial YUV mode Output sequence 2-pixel-pair of every scan line."]
pub type YUV_SM_R = crate::FieldReader<YUV_SM_A>;
#[doc = "Serial YUV mode Output sequence 2-pixel-pair of every scan line.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum YUV_SM_A {
    #[doc = "0: YUYV"]
    YUYV = 0,
    #[doc = "1: YVYU"]
    YVYU = 1,
    #[doc = "2: UYVY"]
    UYVY = 2,
    #[doc = "3: VYUY"]
    VYUY = 3,
}
impl From<YUV_SM_A> for u8 {
    #[inline(always)]
    fn from(variant: YUV_SM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for YUV_SM_A {
    type Ux = u8;
}
impl YUV_SM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> YUV_SM_A {
        match self.bits {
            0 => YUV_SM_A::YUYV,
            1 => YUV_SM_A::YVYU,
            2 => YUV_SM_A::UYVY,
            3 => YUV_SM_A::VYUY,
            _ => unreachable!(),
        }
    }
    #[doc = "YUYV"]
    #[inline(always)]
    pub fn is_yuyv(&self) -> bool {
        *self == YUV_SM_A::YUYV
    }
    #[doc = "YVYU"]
    #[inline(always)]
    pub fn is_yvyu(&self) -> bool {
        *self == YUV_SM_A::YVYU
    }
    #[doc = "UYVY"]
    #[inline(always)]
    pub fn is_uyvy(&self) -> bool {
        *self == YUV_SM_A::UYVY
    }
    #[doc = "VYUY"]
    #[inline(always)]
    pub fn is_vyuy(&self) -> bool {
        *self == YUV_SM_A::VYUY
    }
}
#[doc = "Field `yuv_sm` writer - Serial YUV mode Output sequence 2-pixel-pair of every scan line."]
pub type YUV_SM_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, YUV_SM_A>;
impl<'a, REG> YUV_SM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "YUYV"]
    #[inline(always)]
    pub fn yuyv(self) -> &'a mut crate::W<REG> {
        self.variant(YUV_SM_A::YUYV)
    }
    #[doc = "YVYU"]
    #[inline(always)]
    pub fn yvyu(self) -> &'a mut crate::W<REG> {
        self.variant(YUV_SM_A::YVYU)
    }
    #[doc = "UYVY"]
    #[inline(always)]
    pub fn uyvy(self) -> &'a mut crate::W<REG> {
        self.variant(YUV_SM_A::UYVY)
    }
    #[doc = "VYUY"]
    #[inline(always)]
    pub fn vyuy(self) -> &'a mut crate::W<REG> {
        self.variant(YUV_SM_A::VYUY)
    }
}
#[doc = "Field `rgb888_even_order` reader - Serial RGB888 mode Output sequence at even lines of the panel (line 2, 4, 6, 8...)."]
pub type RGB888_EVEN_ORDER_R = crate::FieldReader<RGB888_EVEN_ORDER_A>;
#[doc = "Serial RGB888 mode Output sequence at even lines of the panel (line 2, 4, 6, 8...).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RGB888_EVEN_ORDER_A {
    #[doc = "0: R -> G -> B"]
    RGB = 0,
    #[doc = "1: B -> R -> G"]
    BRG = 1,
    #[doc = "2: G -> B -> R"]
    GBR = 2,
}
impl From<RGB888_EVEN_ORDER_A> for u8 {
    #[inline(always)]
    fn from(variant: RGB888_EVEN_ORDER_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RGB888_EVEN_ORDER_A {
    type Ux = u8;
}
impl RGB888_EVEN_ORDER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RGB888_EVEN_ORDER_A> {
        match self.bits {
            0 => Some(RGB888_EVEN_ORDER_A::RGB),
            1 => Some(RGB888_EVEN_ORDER_A::BRG),
            2 => Some(RGB888_EVEN_ORDER_A::GBR),
            _ => None,
        }
    }
    #[doc = "R -> G -> B"]
    #[inline(always)]
    pub fn is_rgb(&self) -> bool {
        *self == RGB888_EVEN_ORDER_A::RGB
    }
    #[doc = "B -> R -> G"]
    #[inline(always)]
    pub fn is_brg(&self) -> bool {
        *self == RGB888_EVEN_ORDER_A::BRG
    }
    #[doc = "G -> B -> R"]
    #[inline(always)]
    pub fn is_gbr(&self) -> bool {
        *self == RGB888_EVEN_ORDER_A::GBR
    }
}
#[doc = "Field `rgb888_even_order` writer - Serial RGB888 mode Output sequence at even lines of the panel (line 2, 4, 6, 8...)."]
pub type RGB888_EVEN_ORDER_W<'a, REG> = crate::FieldWriter<'a, REG, 2, RGB888_EVEN_ORDER_A>;
impl<'a, REG> RGB888_EVEN_ORDER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "R -> G -> B"]
    #[inline(always)]
    pub fn rgb(self) -> &'a mut crate::W<REG> {
        self.variant(RGB888_EVEN_ORDER_A::RGB)
    }
    #[doc = "B -> R -> G"]
    #[inline(always)]
    pub fn brg(self) -> &'a mut crate::W<REG> {
        self.variant(RGB888_EVEN_ORDER_A::BRG)
    }
    #[doc = "G -> B -> R"]
    #[inline(always)]
    pub fn gbr(self) -> &'a mut crate::W<REG> {
        self.variant(RGB888_EVEN_ORDER_A::GBR)
    }
}
#[doc = "Field `rgb888_odd_order` reader - Serial RGB888 mode Output sequence at odd lines of the panel (line 1, 3, 5, 7...)."]
pub type RGB888_ODD_ORDER_R = crate::FieldReader<RGB888_ODD_ORDER_A>;
#[doc = "Serial RGB888 mode Output sequence at odd lines of the panel (line 1, 3, 5, 7...).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RGB888_ODD_ORDER_A {
    #[doc = "0: R -> G -> B"]
    RGB = 0,
    #[doc = "1: B -> R -> G"]
    BRG = 1,
    #[doc = "2: G -> B -> R"]
    GBR = 2,
}
impl From<RGB888_ODD_ORDER_A> for u8 {
    #[inline(always)]
    fn from(variant: RGB888_ODD_ORDER_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RGB888_ODD_ORDER_A {
    type Ux = u8;
}
impl RGB888_ODD_ORDER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RGB888_ODD_ORDER_A> {
        match self.bits {
            0 => Some(RGB888_ODD_ORDER_A::RGB),
            1 => Some(RGB888_ODD_ORDER_A::BRG),
            2 => Some(RGB888_ODD_ORDER_A::GBR),
            _ => None,
        }
    }
    #[doc = "R -> G -> B"]
    #[inline(always)]
    pub fn is_rgb(&self) -> bool {
        *self == RGB888_ODD_ORDER_A::RGB
    }
    #[doc = "B -> R -> G"]
    #[inline(always)]
    pub fn is_brg(&self) -> bool {
        *self == RGB888_ODD_ORDER_A::BRG
    }
    #[doc = "G -> B -> R"]
    #[inline(always)]
    pub fn is_gbr(&self) -> bool {
        *self == RGB888_ODD_ORDER_A::GBR
    }
}
#[doc = "Field `rgb888_odd_order` writer - Serial RGB888 mode Output sequence at odd lines of the panel (line 1, 3, 5, 7...)."]
pub type RGB888_ODD_ORDER_W<'a, REG> = crate::FieldWriter<'a, REG, 2, RGB888_ODD_ORDER_A>;
impl<'a, REG> RGB888_ODD_ORDER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "R -> G -> B"]
    #[inline(always)]
    pub fn rgb(self) -> &'a mut crate::W<REG> {
        self.variant(RGB888_ODD_ORDER_A::RGB)
    }
    #[doc = "B -> R -> G"]
    #[inline(always)]
    pub fn brg(self) -> &'a mut crate::W<REG> {
        self.variant(RGB888_ODD_ORDER_A::BRG)
    }
    #[doc = "G -> B -> R"]
    #[inline(always)]
    pub fn gbr(self) -> &'a mut crate::W<REG> {
        self.variant(RGB888_ODD_ORDER_A::GBR)
    }
}
#[doc = "Field `hv_mode` reader - Set the HV mode of LCD controller"]
pub type HV_MODE_R = crate::FieldReader<HV_MODE_A>;
#[doc = "Set the HV mode of LCD controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HV_MODE_A {
    #[doc = "0: 24-bit/1-cycle parallel mode"]
    PARALLEL = 0,
    #[doc = "8: 8-bit/3-cycle RGB serial mode (RGB888)"]
    RGB888 = 8,
    #[doc = "10: 8-bit/4-cycle Dummy RGB (DRGB)"]
    DRGB = 10,
    #[doc = "11: 8-bit/4-cycle RGB Dummy (RGBD)"]
    RGBD = 11,
    #[doc = "12: 8-bit/2-cycle YUV serial mode (CCIR656)"]
    CCIR656 = 12,
}
impl From<HV_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: HV_MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HV_MODE_A {
    type Ux = u8;
}
impl HV_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HV_MODE_A> {
        match self.bits {
            0 => Some(HV_MODE_A::PARALLEL),
            8 => Some(HV_MODE_A::RGB888),
            10 => Some(HV_MODE_A::DRGB),
            11 => Some(HV_MODE_A::RGBD),
            12 => Some(HV_MODE_A::CCIR656),
            _ => None,
        }
    }
    #[doc = "24-bit/1-cycle parallel mode"]
    #[inline(always)]
    pub fn is_parallel(&self) -> bool {
        *self == HV_MODE_A::PARALLEL
    }
    #[doc = "8-bit/3-cycle RGB serial mode (RGB888)"]
    #[inline(always)]
    pub fn is_rgb888(&self) -> bool {
        *self == HV_MODE_A::RGB888
    }
    #[doc = "8-bit/4-cycle Dummy RGB (DRGB)"]
    #[inline(always)]
    pub fn is_drgb(&self) -> bool {
        *self == HV_MODE_A::DRGB
    }
    #[doc = "8-bit/4-cycle RGB Dummy (RGBD)"]
    #[inline(always)]
    pub fn is_rgbd(&self) -> bool {
        *self == HV_MODE_A::RGBD
    }
    #[doc = "8-bit/2-cycle YUV serial mode (CCIR656)"]
    #[inline(always)]
    pub fn is_ccir656(&self) -> bool {
        *self == HV_MODE_A::CCIR656
    }
}
#[doc = "Field `hv_mode` writer - Set the HV mode of LCD controller"]
pub type HV_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, HV_MODE_A>;
impl<'a, REG> HV_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "24-bit/1-cycle parallel mode"]
    #[inline(always)]
    pub fn parallel(self) -> &'a mut crate::W<REG> {
        self.variant(HV_MODE_A::PARALLEL)
    }
    #[doc = "8-bit/3-cycle RGB serial mode (RGB888)"]
    #[inline(always)]
    pub fn rgb888(self) -> &'a mut crate::W<REG> {
        self.variant(HV_MODE_A::RGB888)
    }
    #[doc = "8-bit/4-cycle Dummy RGB (DRGB)"]
    #[inline(always)]
    pub fn drgb(self) -> &'a mut crate::W<REG> {
        self.variant(HV_MODE_A::DRGB)
    }
    #[doc = "8-bit/4-cycle RGB Dummy (RGBD)"]
    #[inline(always)]
    pub fn rgbd(self) -> &'a mut crate::W<REG> {
        self.variant(HV_MODE_A::RGBD)
    }
    #[doc = "8-bit/2-cycle YUV serial mode (CCIR656)"]
    #[inline(always)]
    pub fn ccir656(self) -> &'a mut crate::W<REG> {
        self.variant(HV_MODE_A::CCIR656)
    }
}
impl R {
    #[doc = "Bit 19 - LCD convert source from RGB to YUV.\n\nOnly valid when HV mode is “1100”."]
    #[inline(always)]
    pub fn ccir_csc_dis(&self) -> CCIR_CSC_DIS_R {
        CCIR_CSC_DIS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Set the delay line mode."]
    #[inline(always)]
    pub fn yuv_eav_sav_f_line_dly(&self) -> YUV_EAV_SAV_F_LINE_DLY_R {
        YUV_EAV_SAV_F_LINE_DLY_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Serial YUV mode Output sequence 2-pixel-pair of every scan line."]
    #[inline(always)]
    pub fn yuv_sm(&self) -> YUV_SM_R {
        YUV_SM_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Serial RGB888 mode Output sequence at even lines of the panel (line 2, 4, 6, 8...)."]
    #[inline(always)]
    pub fn rgb888_even_order(&self) -> RGB888_EVEN_ORDER_R {
        RGB888_EVEN_ORDER_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Serial RGB888 mode Output sequence at odd lines of the panel (line 1, 3, 5, 7...)."]
    #[inline(always)]
    pub fn rgb888_odd_order(&self) -> RGB888_ODD_ORDER_R {
        RGB888_ODD_ORDER_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:31 - Set the HV mode of LCD controller"]
    #[inline(always)]
    pub fn hv_mode(&self) -> HV_MODE_R {
        HV_MODE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 19 - LCD convert source from RGB to YUV.\n\nOnly valid when HV mode is “1100”."]
    #[inline(always)]
    #[must_use]
    pub fn ccir_csc_dis(&mut self) -> CCIR_CSC_DIS_W<LCD_HV_IF_SPEC> {
        CCIR_CSC_DIS_W::new(self, 19)
    }
    #[doc = "Bits 20:21 - Set the delay line mode."]
    #[inline(always)]
    #[must_use]
    pub fn yuv_eav_sav_f_line_dly(&mut self) -> YUV_EAV_SAV_F_LINE_DLY_W<LCD_HV_IF_SPEC> {
        YUV_EAV_SAV_F_LINE_DLY_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Serial YUV mode Output sequence 2-pixel-pair of every scan line."]
    #[inline(always)]
    #[must_use]
    pub fn yuv_sm(&mut self) -> YUV_SM_W<LCD_HV_IF_SPEC> {
        YUV_SM_W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Serial RGB888 mode Output sequence at even lines of the panel (line 2, 4, 6, 8...)."]
    #[inline(always)]
    #[must_use]
    pub fn rgb888_even_order(&mut self) -> RGB888_EVEN_ORDER_W<LCD_HV_IF_SPEC> {
        RGB888_EVEN_ORDER_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Serial RGB888 mode Output sequence at odd lines of the panel (line 1, 3, 5, 7...)."]
    #[inline(always)]
    #[must_use]
    pub fn rgb888_odd_order(&mut self) -> RGB888_ODD_ORDER_W<LCD_HV_IF_SPEC> {
        RGB888_ODD_ORDER_W::new(self, 26)
    }
    #[doc = "Bits 28:31 - Set the HV mode of LCD controller"]
    #[inline(always)]
    #[must_use]
    pub fn hv_mode(&mut self) -> HV_MODE_W<LCD_HV_IF_SPEC> {
        HV_MODE_W::new(self, 28)
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
#[doc = "LCD HV Panel Interface Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_hv_if::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_hv_if::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_HV_IF_SPEC;
impl crate::RegisterSpec for LCD_HV_IF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_hv_if::R`](R) reader structure"]
impl crate::Readable for LCD_HV_IF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_hv_if::W`](W) writer structure"]
impl crate::Writable for LCD_HV_IF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_hv_if to value 0"]
impl crate::Resettable for LCD_HV_IF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
