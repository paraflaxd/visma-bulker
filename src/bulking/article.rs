use std::{fs, path::Path};

use fantoccini::{Client, Locator};

use crate::visma_navigation::goto_article_menu;

pub struct Article<'a> {
    pub number: String,
    pub name: String,
    pub price_vat: f32,
    pub image_path: &'a Path
}

const EDITOR_BTN: Locator<'static> = Locator::XPath("//*[@id=\"create-article\"]");
const NUMBER_INPUT: Locator<'static> = Locator::XPath("//*[@id=\"articleno\"]");
const NAME_INPUT: Locator<'static> = Locator::XPath("//*[@id=\"SV_Name\"]");
const PRICE_VAT_INPUT: Locator<'static> = Locator::XPath("//*[@id=\"gross-price\"]");
const MAX_HEIGHT: Locator<'static> = Locator::Id("max-height");
const MAX_WIDTH: Locator<'static> = Locator::Id("max_width");

pub async fn add_articles<'a>(c: &Client, articles: &Vec<&Article<'a>>) -> Result<(), fantoccini::error::CmdError> {
    goto_article_menu(&c).await?;
    
    c.find(EDITOR_BTN).await?.click().await?;

    for a in articles {
        send_keys(c, NUMBER_INPUT, &a.number).await?;
        send_keys(c, NAME_INPUT, &a.name).await?;
        send_keys(c, PRICE_VAT_INPUT, &a.price_vat.to_string()).await?;
        upload_image(c, &a.image_path).await?;
        panic!()
    }
    
    Ok(())
}

async fn send_keys<'a>(c: &Client, locator: Locator<'a>, text: &str) -> Result<(), fantoccini::error::CmdError> {
    c.find(locator).await?.send_keys(&text).await?;
    Ok(())
}

async fn upload_image(c: &Client, image: &Path) -> Result<(), fantoccini::error::CmdError> {
    let image_id = String::from("00000000-0000-0000-0000-000000000000");
    //let max_height: String = c.find(MAX_HEIGHT).await?.attr("value").await?.expect("Couldn't get max height.");
    //let max_width: String = c.find(MAX_WIDTH).await?.attr("value").await?.expect("Couldn't get max width.");
    let max_height = String::from("161");
    let max_width = String::from("161");

    // TODO: THIS
    let image_bytes = fs::read(image).expect("Couldn't read image file");
    
    let request: String = format!("
        var data = new FormData();

        data.append('ImageId', {image_id});
        data.append('MaxHeight', {max_height});
        data.append('MaxWidth', {max_width});
        data.append('MaxFileSize', 5242880);

        var data = {image_bytes}
        var bytes = new Uint8Array(data.length / 2);

        for (var i = 0; i < data.length; i += 2) {{
            bytes[i / 2] = parseInt(data.substring(i, i + 2), /* base = */ 16);
        }}

        let blob = new Blob([bytes], {{type: 'image/jpeg'}})
        let file = new File(blob)
        data.append('file', file);

        Ajax(
            SITE_ROOT + '/System/ImageUpload/',
            data,
            function (response) {{
                if (self._validateResponse(response, maxFileSize)) {{
                    applyFileUploadChanges(response);
                }}
            }},
            'POST',
            true,
            undefined,
            undefined,
            false,
            undefined,
            false
        );
    ");

    c.execute_async(&request, vec![]).await?;

    Ok(())
}

/*ajax({
    url: n,
    processData: h,
    data: t,
    success: i,
    async: u,
    cache: f,
    context: e,
    contentType: o,
    error: s,
    fail: function () {
      var n = this;
      $(n).f5Modal({
        type: 'error',
        modalContent: resources_Common.imsg_Fail
      })
    }
  })
 *
 * $(curImageContainerSelector).append('<input class="old-image-file" type="hidden" value=' + $(curImageContainerSelector).children('input#uploadedImageFile').val() + '>');

                var data = new FormData();

                data.append("ImageId", $(curImageContainerSelector).children('input#uploadedImageFile').val());
                data.append("MaxHeight", maxHeight);
                data.append("MaxWidth", maxWidth);
                data.append("MaxFileSize", maxFileSize);
                data.append("file", file);

                Ajax(
                    SITE_ROOT + '/System/ImageUpload/',
                    data,
                    function (response) {
                        if (self._validateResponse(response, maxFileSize)) {
                            applyFileUploadChanges(response);
                        }
                    },
                    "POST",
                    true,
                    undefined,
                    undefined,
                    false,
                    undefined,
                    false
                );
            });
 *
 */
// THIS IS THE IMAGE UPLOAD FUNCTION THAT IS BEING RUN ON THE WEBSITE
// data-image-settings="/161/161"
// SITE_ROOT=https://www.yourvismawebsite.com/admin
// SHARD_NAME=11
// 
/*
$.widget("f5.f5ImageDragnDropUpload", {
        options: {
            elementSelector: "",
            postUrl: "/System/ImageUpload/",
            allowedExtensionsList: ['jpg', 'jpeg', 'png', 'gif', 'bmp']
        },
        _create: function () {
            var self = this;
            var curImageContainerSelector = self.options.elementSelector + " .curImage";
            var maxHeight = $(self.options.elementSelector).find('#max-height').val();
            var maxWidth = $(self.options.elementSelector).find('#max-width').val();
            var maxFileSize = 5242880;
            var fileInput = $(self.options.elementSelector + " .image-upload-input")[0];
            var deleteLink = $(self.options.elementSelector + " .imageUploaderRemove");

            if ($(this.element).find('#max-file-size').length == 1) {
                maxFileSize = $(this.element).find('#max-file-size').val();
            }

            function applyFileUploadChanges(response) {
                var imageSettings = $(curImageContainerSelector).data('image-settings');
                var imageUrl = SITE_ROOT + '/cdn/img/' + SHARD_NAME + '/' + response + imageSettings;
                var containerId = '#' + $(curImageContainerSelector).attr('id');
                
                $(containerId).children('input.file-name-input').addClass('new-image-file');
                $(containerId).children('img').first().attr('src', imageUrl);
                $(containerId).children('input.file-name-input').val(response);

                $(containerId).children('input.file-name-input').trigger("change");

                deleteLink.show();
                $(containerId).children('.imageUploader').first().removeClass("centered");
            }

            $(curImageContainerSelector).on("mouseenter",
                function (e) {
                    $(self.options.elementSelector + " .mask", $(e.currentTarget)).fadeIn(100);
                    $(self.options.elementSelector + " .imageUploader", $(e.currentTarget)).show();
                    if ($(self.options.elementSelector + " input[name^=uploaded-image-file]", $(e.currentTarget)).val() != "00000000-0000-0000-0000-000000000000") {
                        $(self.options.elementSelector + " .imageUploaderRemove", $(e.currentTarget)).show();
                        $(self.options.elementSelector + ".imageUploader", $(e.currentTarget)).removeClass("centered");
                    } else {
                        $(self.options.elementSelector + " .imageUploader", $(e.currentTarget)).addClass("centered");
                    }
                }).on("mouseleave",
                    function (e) {
                        var toElement = $(e.toElement);
                        if (toElement.length == 0) {
                            toElement = $(e.relatedTarget);
                        }
                        if (toElement.attr("name") != "imageFile") {
                            self._hideMask();
                        }
                    }
                );
            $(self.options.elementSelector + " .imageUploader").on("click", function (e) {
                $(fileInput).trigger("click");
            });

            $(fileInput).on("change", function (e) {
                if (e && e.preventDefault) {
                    e.preventDefault();
                }

                if (IsAjaxActivityInDialog()) { // We should only continue if there are no ajax activity
                    return false;
                }

                var file = $(fileInput)[0].files[0];

                if (!file) {
                    return false;
                }

                if (!self._isFileSizeValid(file.size, maxFileSize)) {
                    self._displaySizeExceededErrorMessage(maxFileSize);
                    return false;
                }

                if (!self._isFileTypeValid(self._getFileExtension(file.name), self.options.allowedExtensionsList)) {
                    self._displayErrorMessage(self._getFileTypeErrorMessage(self.options.allowedExtensionsList));
                    return false;
                }

                ShowDialogAjaxSpinner();

                $(curImageContainerSelector).append('<input class="old-image-file" type="hidden" value=' + $(curImageContainerSelector).children('input#uploadedImageFile').val() + '>');

                var data = new FormData();

                data.append("ImageId", $(curImageContainerSelector).children('input#uploadedImageFile').val());
                data.append("MaxHeight", maxHeight);
                data.append("MaxWidth", maxWidth);
                data.append("MaxFileSize", maxFileSize);
                data.append("file", file);

                Ajax(
                    SITE_ROOT + '/System/ImageUpload/',
                    data,
                    function (response) {
                        if (self._validateResponse(response, maxFileSize)) {
                            applyFileUploadChanges(response);
                        }
                    },
                    "POST",
                    true,
                    undefined,
                    undefined,
                    false,
                    undefined,
                    false
                );
            });
*/
