######################################
#
# dm-big-muff
#
######################################

DM_BIG_MUFF_VERSION = <SHA>
DM_BIG_MUFF_SITE = https://github.com/davemollen/dm-BigMuff.git
DM_BIG_MUFF_SITE_METHOD = git
DM_BIG_MUFF_BUNDLES = dm-BigMuff.lv2

define DM_BIG_MUFF_BUILD_CMDS
	~/.cargo/bin/rustup default nightly

	rm -f $(@D)/lv2/dm-BigMuff.lv2/libdm_big_muff.so
	(cd $(@D)/lv2 && \
		~/.cargo/bin/cargo build $(MOD_PLUGIN_BUILDER_RUST_BUILD_FLAGS))

	~/.cargo/bin/rustup default stable
endef

define DM_BIG_MUFF_INSTALL_TARGET_CMDS
	$(INSTALL) -d $(TARGET_DIR)/usr/lib/lv2
	cp -rv $(@D)/lv2/dm-BigMuff.lv2 $(TARGET_DIR)/usr/lib/lv2/
	$(INSTALL) -m 644 $(@D)/lv2/target/$(MOD_PLUGIN_BUILDER_RUST_TARGET)/release/libdm_big_muff.so $(TARGET_DIR)/usr/lib/lv2/dm-BigMuff.lv2/
endef

$(eval $(generic-package))
