extern "C" {

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineContextMenuData_staticMetaObject(
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineDownloadItem_staticMetaObject(
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineFullScreenRequest_staticMetaObject(
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_staticMetaObject(
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineProfile_staticMetaObject(
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineView_staticMetaObject(
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineCertificateError_QWebEngineCertificateError(
        error: ::std::os::raw::c_int,
        url: *const ::qt_core::QUrl,
        overridable: bool,
        errorDescription: *const ::qt_core::QString,
    ) -> *mut crate::QWebEngineCertificateError;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineCertificateError_dQWebEngineCertificateError(
        this_ptr: *mut crate::QWebEngineCertificateError,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineCertificateError_error(
        this_ptr: *const crate::QWebEngineCertificateError,
    ) -> crate::q_web_engine_certificate_error::Error;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineCertificateError_url(
        this_ptr: *const crate::QWebEngineCertificateError,
    ) -> *mut ::qt_core::QUrl;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineCertificateError_isOverridable(
        this_ptr: *const crate::QWebEngineCertificateError,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineCertificateError_errorDescription(
        this_ptr: *const crate::QWebEngineCertificateError,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineCertificateError_QWebEngineCertificateError1(
        other: *const crate::QWebEngineCertificateError,
    ) -> *mut crate::QWebEngineCertificateError;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineCertificateError_operator_(
        this_ptr: *mut crate::QWebEngineCertificateError,
        other: *const crate::QWebEngineCertificateError,
    ) -> *mut crate::QWebEngineCertificateError;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineCertificateError_defer(
        this_ptr: *mut crate::QWebEngineCertificateError,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineCertificateError_deferred(
        this_ptr: *const crate::QWebEngineCertificateError,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineCertificateError_rejectCertificate(
        this_ptr: *mut crate::QWebEngineCertificateError,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineCertificateError_ignoreCertificateError(
        this_ptr: *mut crate::QWebEngineCertificateError,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineCertificateError_answered(
        this_ptr: *const crate::QWebEngineCertificateError,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineClientCertificateSelection_QWebEngineClientCertificateSelection(
        arg1: *const crate::QWebEngineClientCertificateSelection,
    ) -> *mut crate::QWebEngineClientCertificateSelection;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineClientCertificateSelection_dQWebEngineClientCertificateSelection(
        this_ptr: *mut crate::QWebEngineClientCertificateSelection,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineClientCertificateSelection_operator_(
        this_ptr: *mut crate::QWebEngineClientCertificateSelection,
        arg1: *const crate::QWebEngineClientCertificateSelection,
    ) -> *mut crate::QWebEngineClientCertificateSelection;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineClientCertificateSelection_host(
        this_ptr: *const crate::QWebEngineClientCertificateSelection,
    ) -> *mut ::qt_core::QUrl;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineClientCertificateSelection_selectNone(
        this_ptr: *mut crate::QWebEngineClientCertificateSelection,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineContextMenuData_QWebEngineContextMenuData(
    ) -> *mut crate::QWebEngineContextMenuData;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineContextMenuData_QWebEngineContextMenuData1(
        other: *const crate::QWebEngineContextMenuData,
    ) -> *mut crate::QWebEngineContextMenuData;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineContextMenuData_operator_(
        this_ptr: *mut crate::QWebEngineContextMenuData,
        other: *const crate::QWebEngineContextMenuData,
    ) -> *mut crate::QWebEngineContextMenuData;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineContextMenuData_dQWebEngineContextMenuData(
        this_ptr: *mut crate::QWebEngineContextMenuData,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineContextMenuData_isValid(
        this_ptr: *const crate::QWebEngineContextMenuData,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineContextMenuData_position(
        this_ptr: *const crate::QWebEngineContextMenuData,
    ) -> *mut ::qt_core::QPoint;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineContextMenuData_selectedText(
        this_ptr: *const crate::QWebEngineContextMenuData,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineContextMenuData_linkText(
        this_ptr: *const crate::QWebEngineContextMenuData,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineContextMenuData_linkUrl(
        this_ptr: *const crate::QWebEngineContextMenuData,
    ) -> *mut ::qt_core::QUrl;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineContextMenuData_mediaUrl(
        this_ptr: *const crate::QWebEngineContextMenuData,
    ) -> *mut ::qt_core::QUrl;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineContextMenuData_mediaType(
        this_ptr: *const crate::QWebEngineContextMenuData,
    ) -> crate::q_web_engine_context_menu_data::MediaType;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineContextMenuData_isContentEditable(
        this_ptr: *const crate::QWebEngineContextMenuData,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineContextMenuData_misspelledWord(
        this_ptr: *const crate::QWebEngineContextMenuData,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineContextMenuData_spellCheckerSuggestions(
        this_ptr: *const crate::QWebEngineContextMenuData,
    ) -> *mut ::qt_core::QStringList;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineContextMenuData_mediaFlags(
        this_ptr: *const crate::QWebEngineContextMenuData,
    ) -> ::std::os::raw::c_int;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineContextMenuData_editFlags(
        this_ptr: *const crate::QWebEngineContextMenuData,
    ) -> ::std::os::raw::c_int;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineDownloadItem_metaObject(
        this_ptr: *const crate::QWebEngineDownloadItem,
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineDownloadItem_qt_metacast(
        this_ptr: *mut crate::QWebEngineDownloadItem,
        arg1: *const ::std::os::raw::c_char,
    ) -> *mut ::std::ffi::c_void;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineDownloadItem_qt_metacall(
        this_ptr: *mut crate::QWebEngineDownloadItem,
        arg1: ::qt_core::q_meta_object::Call,
        arg2: ::std::os::raw::c_int,
        arg3: *mut *mut ::std::ffi::c_void,
    ) -> ::std::os::raw::c_int;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineDownloadItem_tr(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineDownloadItem_trUtf8(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineDownloadItem_dQWebEngineDownloadItem(
        this_ptr: *mut crate::QWebEngineDownloadItem,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineDownloadItem_id(
        this_ptr: *const crate::QWebEngineDownloadItem,
    ) -> u32;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineDownloadItem_state(
        this_ptr: *const crate::QWebEngineDownloadItem,
    ) -> crate::q_web_engine_download_item::DownloadState;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineDownloadItem_totalBytes(
        this_ptr: *const crate::QWebEngineDownloadItem,
    ) -> i64;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineDownloadItem_receivedBytes(
        this_ptr: *const crate::QWebEngineDownloadItem,
    ) -> i64;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineDownloadItem_url(
        this_ptr: *const crate::QWebEngineDownloadItem,
    ) -> *mut ::qt_core::QUrl;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineDownloadItem_mimeType(
        this_ptr: *const crate::QWebEngineDownloadItem,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineDownloadItem_path(
        this_ptr: *const crate::QWebEngineDownloadItem,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineDownloadItem_setPath(
        this_ptr: *mut crate::QWebEngineDownloadItem,
        path: *const ::qt_core::QString,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineDownloadItem_isFinished(
        this_ptr: *const crate::QWebEngineDownloadItem,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineDownloadItem_isPaused(
        this_ptr: *const crate::QWebEngineDownloadItem,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineDownloadItem_savePageFormat(
        this_ptr: *const crate::QWebEngineDownloadItem,
    ) -> crate::q_web_engine_download_item::SavePageFormat;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineDownloadItem_setSavePageFormat(
        this_ptr: *mut crate::QWebEngineDownloadItem,
        format: crate::q_web_engine_download_item::SavePageFormat,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineDownloadItem_type(
        this_ptr: *const crate::QWebEngineDownloadItem,
    ) -> crate::q_web_engine_download_item::DownloadType;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineDownloadItem_interruptReason(
        this_ptr: *const crate::QWebEngineDownloadItem,
    ) -> crate::q_web_engine_download_item::DownloadInterruptReason;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineDownloadItem_interruptReasonString(
        this_ptr: *const crate::QWebEngineDownloadItem,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineDownloadItem_isSavePageDownload(
        this_ptr: *const crate::QWebEngineDownloadItem,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineDownloadItem_suggestedFileName(
        this_ptr: *const crate::QWebEngineDownloadItem,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineDownloadItem_downloadDirectory(
        this_ptr: *const crate::QWebEngineDownloadItem,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineDownloadItem_setDownloadDirectory(
        this_ptr: *mut crate::QWebEngineDownloadItem,
        directory: *const ::qt_core::QString,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineDownloadItem_downloadFileName(
        this_ptr: *const crate::QWebEngineDownloadItem,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineDownloadItem_setDownloadFileName(
        this_ptr: *mut crate::QWebEngineDownloadItem,
        fileName: *const ::qt_core::QString,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineDownloadItem_page(
        this_ptr: *const crate::QWebEngineDownloadItem,
    ) -> *mut crate::QWebEnginePage;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineDownloadItem_accept(
        this_ptr: *mut crate::QWebEngineDownloadItem,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineDownloadItem_cancel(
        this_ptr: *mut crate::QWebEngineDownloadItem,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineDownloadItem_pause(
        this_ptr: *mut crate::QWebEngineDownloadItem,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineDownloadItem_resume(
        this_ptr: *mut crate::QWebEngineDownloadItem,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineFullScreenRequest_reject(
        this_ptr: *mut crate::QWebEngineFullScreenRequest,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineFullScreenRequest_accept(
        this_ptr: *mut crate::QWebEngineFullScreenRequest,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineFullScreenRequest_toggleOn(
        this_ptr: *const crate::QWebEngineFullScreenRequest,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineFullScreenRequest_origin(
        this_ptr: *const crate::QWebEngineFullScreenRequest,
    ) -> *const ::qt_core::QUrl;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineHistoryItem_QWebEngineHistoryItem(
        other: *const crate::QWebEngineHistoryItem,
    ) -> *mut crate::QWebEngineHistoryItem;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineHistoryItem_operator_(
        this_ptr: *mut crate::QWebEngineHistoryItem,
        other: *const crate::QWebEngineHistoryItem,
    ) -> *mut crate::QWebEngineHistoryItem;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineHistoryItem_dQWebEngineHistoryItem(
        this_ptr: *mut crate::QWebEngineHistoryItem,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineHistoryItem_originalUrl(
        this_ptr: *const crate::QWebEngineHistoryItem,
    ) -> *mut ::qt_core::QUrl;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineHistoryItem_url(
        this_ptr: *const crate::QWebEngineHistoryItem,
    ) -> *mut ::qt_core::QUrl;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineHistoryItem_title(
        this_ptr: *const crate::QWebEngineHistoryItem,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineHistoryItem_lastVisited(
        this_ptr: *const crate::QWebEngineHistoryItem,
    ) -> *mut ::qt_core::QDateTime;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineHistoryItem_iconUrl(
        this_ptr: *const crate::QWebEngineHistoryItem,
    ) -> *mut ::qt_core::QUrl;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineHistoryItem_isValid(
        this_ptr: *const crate::QWebEngineHistoryItem,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineHistoryItem_swap(
        this_ptr: *mut crate::QWebEngineHistoryItem,
        other: *mut crate::QWebEngineHistoryItem,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_swap(
        value1: *mut crate::QWebEngineHistoryItem,
        value2: *mut crate::QWebEngineHistoryItem,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineHistory_clear(
        this_ptr: *mut crate::QWebEngineHistory,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineHistory_items(
        this_ptr: *const crate::QWebEngineHistory,
    ) -> *mut crate::QListOfQWebEngineHistoryItem;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineHistory_backItems(
        this_ptr: *const crate::QWebEngineHistory,
        maxItems: ::std::os::raw::c_int,
    ) -> *mut crate::QListOfQWebEngineHistoryItem;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineHistory_forwardItems(
        this_ptr: *const crate::QWebEngineHistory,
        maxItems: ::std::os::raw::c_int,
    ) -> *mut crate::QListOfQWebEngineHistoryItem;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineHistory_canGoBack(
        this_ptr: *const crate::QWebEngineHistory,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineHistory_canGoForward(
        this_ptr: *const crate::QWebEngineHistory,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineHistory_back(
        this_ptr: *mut crate::QWebEngineHistory,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineHistory_forward(
        this_ptr: *mut crate::QWebEngineHistory,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineHistory_goToItem(
        this_ptr: *mut crate::QWebEngineHistory,
        item: *const crate::QWebEngineHistoryItem,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineHistory_backItem(
        this_ptr: *const crate::QWebEngineHistory,
    ) -> *mut crate::QWebEngineHistoryItem;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineHistory_currentItem(
        this_ptr: *const crate::QWebEngineHistory,
    ) -> *mut crate::QWebEngineHistoryItem;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineHistory_forwardItem(
        this_ptr: *const crate::QWebEngineHistory,
    ) -> *mut crate::QWebEngineHistoryItem;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineHistory_itemAt(
        this_ptr: *const crate::QWebEngineHistory,
        i: ::std::os::raw::c_int,
    ) -> *mut crate::QWebEngineHistoryItem;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineHistory_currentItemIndex(
        this_ptr: *const crate::QWebEngineHistory,
    ) -> ::std::os::raw::c_int;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineHistory_count(
        this_ptr: *const crate::QWebEngineHistory,
    ) -> ::std::os::raw::c_int;

    pub fn ctr_qt_web_engine_widgets_ffi_operator__(
        stream: *mut ::qt_core::QDataStream,
        history: *const crate::QWebEngineHistory,
    ) -> *mut ::qt_core::QDataStream;

    pub fn ctr_qt_web_engine_widgets_ffi_operator__1(
        stream: *mut ::qt_core::QDataStream,
        history: *mut crate::QWebEngineHistory,
    ) -> *mut ::qt_core::QDataStream;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_metaObject(
        this_ptr: *const crate::QWebEnginePage,
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_qt_metacast(
        this_ptr: *mut crate::QWebEnginePage,
        arg1: *const ::std::os::raw::c_char,
    ) -> *mut ::std::ffi::c_void;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_qt_metacall(
        this_ptr: *mut crate::QWebEnginePage,
        arg1: ::qt_core::q_meta_object::Call,
        arg2: ::std::os::raw::c_int,
        arg3: *mut *mut ::std::ffi::c_void,
    ) -> ::std::os::raw::c_int;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_tr(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_trUtf8(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_QWebEnginePage(
        parent: *mut ::qt_core::QObject,
    ) -> *mut crate::QWebEnginePage;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_QWebEnginePage1(
        profile: *mut crate::QWebEngineProfile,
        parent: *mut ::qt_core::QObject,
    ) -> *mut crate::QWebEnginePage;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_dQWebEnginePage(
        this_ptr: *mut crate::QWebEnginePage,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_history(
        this_ptr: *const crate::QWebEnginePage,
    ) -> *mut crate::QWebEngineHistory;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_setView(
        this_ptr: *mut crate::QWebEnginePage,
        view: *mut ::qt_widgets::QWidget,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_view(
        this_ptr: *const crate::QWebEnginePage,
    ) -> *mut ::qt_widgets::QWidget;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_hasSelection(
        this_ptr: *const crate::QWebEnginePage,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_selectedText(
        this_ptr: *const crate::QWebEnginePage,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_profile(
        this_ptr: *const crate::QWebEnginePage,
    ) -> *mut crate::QWebEngineProfile;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_action(
        this_ptr: *const crate::QWebEnginePage,
        action: crate::q_web_engine_page::WebAction,
    ) -> *mut ::qt_widgets::QAction;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_triggerAction(
        this_ptr: *mut crate::QWebEnginePage,
        action: crate::q_web_engine_page::WebAction,
        checked: bool,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_replaceMisspelledWord(
        this_ptr: *mut crate::QWebEnginePage,
        replacement: *const ::qt_core::QString,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_event(
        this_ptr: *mut crate::QWebEnginePage,
        arg1: *mut ::qt_core::QEvent,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_createStandardContextMenu(
        this_ptr: *mut crate::QWebEnginePage,
    ) -> *mut ::qt_widgets::QMenu;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_setFeaturePermission(
        this_ptr: *mut crate::QWebEnginePage,
        securityOrigin: *const ::qt_core::QUrl,
        feature: crate::q_web_engine_page::Feature,
        policy: crate::q_web_engine_page::PermissionPolicy,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_load(
        this_ptr: *mut crate::QWebEnginePage,
        url: *const ::qt_core::QUrl,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_download(
        this_ptr: *mut crate::QWebEnginePage,
        url: *const ::qt_core::QUrl,
        filename: *const ::qt_core::QString,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_setHtml(
        this_ptr: *mut crate::QWebEnginePage,
        html: *const ::qt_core::QString,
        baseUrl: *const ::qt_core::QUrl,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_setContent(
        this_ptr: *mut crate::QWebEnginePage,
        data: *const ::qt_core::QByteArray,
        mimeType: *const ::qt_core::QString,
        baseUrl: *const ::qt_core::QUrl,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_title(
        this_ptr: *const crate::QWebEnginePage,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_setUrl(
        this_ptr: *mut crate::QWebEnginePage,
        url: *const ::qt_core::QUrl,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_url(
        this_ptr: *const crate::QWebEnginePage,
    ) -> *mut ::qt_core::QUrl;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_requestedUrl(
        this_ptr: *const crate::QWebEnginePage,
    ) -> *mut ::qt_core::QUrl;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_iconUrl(
        this_ptr: *const crate::QWebEnginePage,
    ) -> *mut ::qt_core::QUrl;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_icon(
        this_ptr: *const crate::QWebEnginePage,
    ) -> *mut ::qt_gui::QIcon;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_zoomFactor(
        this_ptr: *const crate::QWebEnginePage,
    ) -> ::std::os::raw::c_double;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_setZoomFactor(
        this_ptr: *mut crate::QWebEnginePage,
        factor: ::std::os::raw::c_double,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_scrollPosition(
        this_ptr: *const crate::QWebEnginePage,
    ) -> *mut ::qt_core::QPointF;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_contentsSize(
        this_ptr: *const crate::QWebEnginePage,
    ) -> *mut ::qt_core::QSizeF;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_runJavaScript(
        this_ptr: *mut crate::QWebEnginePage,
        scriptSource: *const ::qt_core::QString,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_runJavaScript1(
        this_ptr: *mut crate::QWebEnginePage,
        scriptSource: *const ::qt_core::QString,
        worldId: u32,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_scripts(
        this_ptr: *mut crate::QWebEnginePage,
    ) -> *mut crate::QWebEngineScriptCollection;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_settings(
        this_ptr: *const crate::QWebEnginePage,
    ) -> *mut crate::QWebEngineSettings;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_backgroundColor(
        this_ptr: *const crate::QWebEnginePage,
    ) -> *mut ::qt_gui::QColor;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_setBackgroundColor(
        this_ptr: *mut crate::QWebEnginePage,
        color: *const ::qt_gui::QColor,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_save(
        this_ptr: *const crate::QWebEnginePage,
        filePath: *const ::qt_core::QString,
        format: crate::q_web_engine_download_item::SavePageFormat,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_isAudioMuted(
        this_ptr: *const crate::QWebEnginePage,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_setAudioMuted(
        this_ptr: *mut crate::QWebEnginePage,
        muted: bool,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_recentlyAudible(
        this_ptr: *const crate::QWebEnginePage,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_renderProcessPid(
        this_ptr: *const crate::QWebEnginePage,
    ) -> i64;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_printToPdf(
        this_ptr: *mut crate::QWebEnginePage,
        filePath: *const ::qt_core::QString,
        layout: *const ::qt_gui::QPageLayout,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_setInspectedPage(
        this_ptr: *mut crate::QWebEnginePage,
        page: *mut crate::QWebEnginePage,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_inspectedPage(
        this_ptr: *const crate::QWebEnginePage,
    ) -> *mut crate::QWebEnginePage;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_setDevToolsPage(
        this_ptr: *mut crate::QWebEnginePage,
        page: *mut crate::QWebEnginePage,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_devToolsPage(
        this_ptr: *const crate::QWebEnginePage,
    ) -> *mut crate::QWebEnginePage;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_contextMenuData(
        this_ptr: *const crate::QWebEnginePage,
    ) -> *const crate::QWebEngineContextMenuData;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_lifecycleState(
        this_ptr: *const crate::QWebEnginePage,
    ) -> crate::q_web_engine_page::LifecycleState;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_setLifecycleState(
        this_ptr: *mut crate::QWebEnginePage,
        state: crate::q_web_engine_page::LifecycleState,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_recommendedState(
        this_ptr: *const crate::QWebEnginePage,
    ) -> crate::q_web_engine_page::LifecycleState;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_isVisible(
        this_ptr: *const crate::QWebEnginePage,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_setVisible(
        this_ptr: *mut crate::QWebEnginePage,
        visible: bool,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineProfile_metaObject(
        this_ptr: *const crate::QWebEngineProfile,
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineProfile_qt_metacast(
        this_ptr: *mut crate::QWebEngineProfile,
        arg1: *const ::std::os::raw::c_char,
    ) -> *mut ::std::ffi::c_void;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineProfile_qt_metacall(
        this_ptr: *mut crate::QWebEngineProfile,
        arg1: ::qt_core::q_meta_object::Call,
        arg2: ::std::os::raw::c_int,
        arg3: *mut *mut ::std::ffi::c_void,
    ) -> ::std::os::raw::c_int;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineProfile_tr(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineProfile_trUtf8(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineProfile_QWebEngineProfile(
        parent: *mut ::qt_core::QObject,
    ) -> *mut crate::QWebEngineProfile;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineProfile_QWebEngineProfile1(
        name: *const ::qt_core::QString,
        parent: *mut ::qt_core::QObject,
    ) -> *mut crate::QWebEngineProfile;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineProfile_dQWebEngineProfile(
        this_ptr: *mut crate::QWebEngineProfile,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineProfile_storageName(
        this_ptr: *const crate::QWebEngineProfile,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineProfile_isOffTheRecord(
        this_ptr: *const crate::QWebEngineProfile,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineProfile_persistentStoragePath(
        this_ptr: *const crate::QWebEngineProfile,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineProfile_setPersistentStoragePath(
        this_ptr: *mut crate::QWebEngineProfile,
        path: *const ::qt_core::QString,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineProfile_cachePath(
        this_ptr: *const crate::QWebEngineProfile,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineProfile_setCachePath(
        this_ptr: *mut crate::QWebEngineProfile,
        path: *const ::qt_core::QString,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineProfile_httpUserAgent(
        this_ptr: *const crate::QWebEngineProfile,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineProfile_setHttpUserAgent(
        this_ptr: *mut crate::QWebEngineProfile,
        userAgent: *const ::qt_core::QString,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineProfile_httpCacheType(
        this_ptr: *const crate::QWebEngineProfile,
    ) -> crate::q_web_engine_profile::HttpCacheType;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineProfile_setHttpCacheType(
        this_ptr: *mut crate::QWebEngineProfile,
        arg1: crate::q_web_engine_profile::HttpCacheType,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineProfile_setHttpAcceptLanguage(
        this_ptr: *mut crate::QWebEngineProfile,
        httpAcceptLanguage: *const ::qt_core::QString,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineProfile_httpAcceptLanguage(
        this_ptr: *const crate::QWebEngineProfile,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineProfile_persistentCookiesPolicy(
        this_ptr: *const crate::QWebEngineProfile,
    ) -> crate::q_web_engine_profile::PersistentCookiesPolicy;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineProfile_setPersistentCookiesPolicy(
        this_ptr: *mut crate::QWebEngineProfile,
        arg1: crate::q_web_engine_profile::PersistentCookiesPolicy,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineProfile_httpCacheMaximumSize(
        this_ptr: *const crate::QWebEngineProfile,
    ) -> ::std::os::raw::c_int;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineProfile_setHttpCacheMaximumSize(
        this_ptr: *mut crate::QWebEngineProfile,
        maxSize: ::std::os::raw::c_int,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineProfile_clearAllVisitedLinks(
        this_ptr: *mut crate::QWebEngineProfile,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineProfile_clearVisitedLinks(
        this_ptr: *mut crate::QWebEngineProfile,
        urls: *const ::qt_core::QListOfQUrl,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineProfile_visitedLinksContainsUrl(
        this_ptr: *const crate::QWebEngineProfile,
        url: *const ::qt_core::QUrl,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineProfile_settings(
        this_ptr: *const crate::QWebEngineProfile,
    ) -> *mut crate::QWebEngineSettings;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineProfile_scripts(
        this_ptr: *const crate::QWebEngineProfile,
    ) -> *mut crate::QWebEngineScriptCollection;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineProfile_removeUrlScheme(
        this_ptr: *mut crate::QWebEngineProfile,
        scheme: *const ::qt_core::QByteArray,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineProfile_removeAllUrlSchemeHandlers(
        this_ptr: *mut crate::QWebEngineProfile,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineProfile_clearHttpCache(
        this_ptr: *mut crate::QWebEngineProfile,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineProfile_setSpellCheckLanguages(
        this_ptr: *mut crate::QWebEngineProfile,
        languages: *const ::qt_core::QStringList,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineProfile_spellCheckLanguages(
        this_ptr: *const crate::QWebEngineProfile,
    ) -> *mut ::qt_core::QStringList;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineProfile_setSpellCheckEnabled(
        this_ptr: *mut crate::QWebEngineProfile,
        enabled: bool,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineProfile_isSpellCheckEnabled(
        this_ptr: *const crate::QWebEngineProfile,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineProfile_setUseForGlobalCertificateVerification(
        this_ptr: *mut crate::QWebEngineProfile,
        enabled: bool,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineProfile_isUsedForGlobalCertificateVerification(
        this_ptr: *const crate::QWebEngineProfile,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineProfile_downloadPath(
        this_ptr: *const crate::QWebEngineProfile,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineProfile_setDownloadPath(
        this_ptr: *mut crate::QWebEngineProfile,
        path: *const ::qt_core::QString,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineProfile_defaultProfile(
    ) -> *mut crate::QWebEngineProfile;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineScript_QWebEngineScript(
    ) -> *mut crate::QWebEngineScript;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineScript_QWebEngineScript1(
        other: *const crate::QWebEngineScript,
    ) -> *mut crate::QWebEngineScript;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineScript_dQWebEngineScript(
        this_ptr: *mut crate::QWebEngineScript,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineScript_operator_(
        this_ptr: *mut crate::QWebEngineScript,
        other: *const crate::QWebEngineScript,
    ) -> *mut crate::QWebEngineScript;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineScript_isNull(
        this_ptr: *const crate::QWebEngineScript,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineScript_name(
        this_ptr: *const crate::QWebEngineScript,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineScript_setName(
        this_ptr: *mut crate::QWebEngineScript,
        arg1: *const ::qt_core::QString,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineScript_sourceCode(
        this_ptr: *const crate::QWebEngineScript,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineScript_setSourceCode(
        this_ptr: *mut crate::QWebEngineScript,
        arg1: *const ::qt_core::QString,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineScript_injectionPoint(
        this_ptr: *const crate::QWebEngineScript,
    ) -> crate::q_web_engine_script::InjectionPoint;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineScript_setInjectionPoint(
        this_ptr: *mut crate::QWebEngineScript,
        arg1: crate::q_web_engine_script::InjectionPoint,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineScript_worldId(
        this_ptr: *const crate::QWebEngineScript,
    ) -> u32;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineScript_setWorldId(
        this_ptr: *mut crate::QWebEngineScript,
        arg1: u32,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineScript_runsOnSubFrames(
        this_ptr: *const crate::QWebEngineScript,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineScript_setRunsOnSubFrames(
        this_ptr: *mut crate::QWebEngineScript,
        on: bool,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineScript_operator__(
        this_ptr: *const crate::QWebEngineScript,
        other: *const crate::QWebEngineScript,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineScript_swap(
        this_ptr: *mut crate::QWebEngineScript,
        other: *mut crate::QWebEngineScript,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_swap1(
        value1: *mut crate::QWebEngineScript,
        value2: *mut crate::QWebEngineScript,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_operator__2(
        arg1: *const ::qt_core::QDebug,
        arg2: *const crate::QWebEngineScript,
    ) -> *mut ::qt_core::QDebug;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineScriptCollection_dQWebEngineScriptCollection(
        this_ptr: *mut crate::QWebEngineScriptCollection,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineScriptCollection_isEmpty(
        this_ptr: *const crate::QWebEngineScriptCollection,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineScriptCollection_count(
        this_ptr: *const crate::QWebEngineScriptCollection,
    ) -> ::std::os::raw::c_int;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineScriptCollection_size(
        this_ptr: *const crate::QWebEngineScriptCollection,
    ) -> ::std::os::raw::c_int;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineScriptCollection_contains(
        this_ptr: *const crate::QWebEngineScriptCollection,
        value: *const crate::QWebEngineScript,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineScriptCollection_findScript(
        this_ptr: *const crate::QWebEngineScriptCollection,
        name: *const ::qt_core::QString,
    ) -> *mut crate::QWebEngineScript;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineScriptCollection_findScripts(
        this_ptr: *const crate::QWebEngineScriptCollection,
        name: *const ::qt_core::QString,
    ) -> *mut crate::QListOfQWebEngineScript;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineScriptCollection_insert(
        this_ptr: *mut crate::QWebEngineScriptCollection,
        arg1: *const crate::QWebEngineScript,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineScriptCollection_insert1(
        this_ptr: *mut crate::QWebEngineScriptCollection,
        list: *const crate::QListOfQWebEngineScript,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineScriptCollection_remove(
        this_ptr: *mut crate::QWebEngineScriptCollection,
        arg1: *const crate::QWebEngineScript,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineScriptCollection_clear(
        this_ptr: *mut crate::QWebEngineScriptCollection,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineScriptCollection_toList(
        this_ptr: *const crate::QWebEngineScriptCollection,
    ) -> *mut crate::QListOfQWebEngineScript;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineSettings_globalSettings(
    ) -> *mut crate::QWebEngineSettings;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineSettings_defaultSettings(
    ) -> *mut crate::QWebEngineSettings;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineSettings_setFontFamily(
        this_ptr: *mut crate::QWebEngineSettings,
        which: crate::q_web_engine_settings::FontFamily,
        family: *const ::qt_core::QString,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineSettings_fontFamily(
        this_ptr: *const crate::QWebEngineSettings,
        which: crate::q_web_engine_settings::FontFamily,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineSettings_resetFontFamily(
        this_ptr: *mut crate::QWebEngineSettings,
        which: crate::q_web_engine_settings::FontFamily,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineSettings_setFontSize(
        this_ptr: *mut crate::QWebEngineSettings,
        type_: crate::q_web_engine_settings::FontSize,
        size: ::std::os::raw::c_int,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineSettings_fontSize(
        this_ptr: *const crate::QWebEngineSettings,
        type_: crate::q_web_engine_settings::FontSize,
    ) -> ::std::os::raw::c_int;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineSettings_resetFontSize(
        this_ptr: *mut crate::QWebEngineSettings,
        type_: crate::q_web_engine_settings::FontSize,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineSettings_setAttribute(
        this_ptr: *mut crate::QWebEngineSettings,
        attr: crate::q_web_engine_settings::WebAttribute,
        on: bool,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineSettings_testAttribute(
        this_ptr: *const crate::QWebEngineSettings,
        attr: crate::q_web_engine_settings::WebAttribute,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineSettings_resetAttribute(
        this_ptr: *mut crate::QWebEngineSettings,
        attr: crate::q_web_engine_settings::WebAttribute,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineSettings_setDefaultTextEncoding(
        this_ptr: *mut crate::QWebEngineSettings,
        encoding: *const ::qt_core::QString,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineSettings_defaultTextEncoding(
        this_ptr: *const crate::QWebEngineSettings,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineSettings_unknownUrlSchemePolicy(
        this_ptr: *const crate::QWebEngineSettings,
    ) -> crate::q_web_engine_settings::UnknownUrlSchemePolicy;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineSettings_setUnknownUrlSchemePolicy(
        this_ptr: *mut crate::QWebEngineSettings,
        policy: crate::q_web_engine_settings::UnknownUrlSchemePolicy,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineSettings_resetUnknownUrlSchemePolicy(
        this_ptr: *mut crate::QWebEngineSettings,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineView_metaObject(
        this_ptr: *const crate::QWebEngineView,
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineView_qt_metacast(
        this_ptr: *mut crate::QWebEngineView,
        arg1: *const ::std::os::raw::c_char,
    ) -> *mut ::std::ffi::c_void;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineView_qt_metacall(
        this_ptr: *mut crate::QWebEngineView,
        arg1: ::qt_core::q_meta_object::Call,
        arg2: ::std::os::raw::c_int,
        arg3: *mut *mut ::std::ffi::c_void,
    ) -> ::std::os::raw::c_int;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineView_tr(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineView_trUtf8(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineView_QWebEngineView(
        parent: *mut ::qt_widgets::QWidget,
    ) -> *mut crate::QWebEngineView;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineView_dQWebEngineView(
        this_ptr: *mut crate::QWebEngineView,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineView_page(
        this_ptr: *const crate::QWebEngineView,
    ) -> *mut crate::QWebEnginePage;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineView_setPage(
        this_ptr: *mut crate::QWebEngineView,
        page: *mut crate::QWebEnginePage,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineView_load(
        this_ptr: *mut crate::QWebEngineView,
        url: *const ::qt_core::QUrl,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineView_setHtml(
        this_ptr: *mut crate::QWebEngineView,
        html: *const ::qt_core::QString,
        baseUrl: *const ::qt_core::QUrl,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineView_setContent(
        this_ptr: *mut crate::QWebEngineView,
        data: *const ::qt_core::QByteArray,
        mimeType: *const ::qt_core::QString,
        baseUrl: *const ::qt_core::QUrl,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineView_history(
        this_ptr: *const crate::QWebEngineView,
    ) -> *mut crate::QWebEngineHistory;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineView_title(
        this_ptr: *const crate::QWebEngineView,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineView_setUrl(
        this_ptr: *mut crate::QWebEngineView,
        url: *const ::qt_core::QUrl,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineView_url(
        this_ptr: *const crate::QWebEngineView,
    ) -> *mut ::qt_core::QUrl;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineView_iconUrl(
        this_ptr: *const crate::QWebEngineView,
    ) -> *mut ::qt_core::QUrl;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineView_icon(
        this_ptr: *const crate::QWebEngineView,
    ) -> *mut ::qt_gui::QIcon;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineView_hasSelection(
        this_ptr: *const crate::QWebEngineView,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineView_selectedText(
        this_ptr: *const crate::QWebEngineView,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineView_pageAction(
        this_ptr: *const crate::QWebEngineView,
        action: crate::q_web_engine_page::WebAction,
    ) -> *mut ::qt_widgets::QAction;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineView_triggerPageAction(
        this_ptr: *mut crate::QWebEngineView,
        action: crate::q_web_engine_page::WebAction,
        checked: bool,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineView_zoomFactor(
        this_ptr: *const crate::QWebEngineView,
    ) -> ::std::os::raw::c_double;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineView_setZoomFactor(
        this_ptr: *mut crate::QWebEngineView,
        factor: ::std::os::raw::c_double,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineView_sizeHint(
        this_ptr: *const crate::QWebEngineView,
    ) -> *mut ::qt_core::QSize;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineView_settings(
        this_ptr: *const crate::QWebEngineView,
    ) -> *mut crate::QWebEngineSettings;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineView_stop(this_ptr: *mut crate::QWebEngineView);

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineView_back(this_ptr: *mut crate::QWebEngineView);

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineView_forward(
        this_ptr: *mut crate::QWebEngineView,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineView_reload(
        this_ptr: *mut crate::QWebEngineView,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineFullScreenRequest_dQWebEngineFullScreenRequest(
        this_ptr: *mut crate::QWebEngineFullScreenRequest,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineFullScreenRequest_QWebEngineFullScreenRequest1(
        other: *const crate::QWebEngineFullScreenRequest,
    ) -> *mut crate::QWebEngineFullScreenRequest;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_QWebEnginePage2(
    ) -> *mut crate::QWebEnginePage;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineProfile_QWebEngineProfile2(
    ) -> *mut crate::QWebEngineProfile;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineView_QWebEngineView1(
    ) -> *mut crate::QWebEngineView;

    /// <p>Constructs an empty list.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#QList">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Constructs an empty list.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_QList(
    ) -> *mut crate::QListOfQWebEngineHistoryItem;

    /// <p>Constructs an empty list.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#QList">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Constructs an empty list.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_QList(
    ) -> *mut crate::QListOfQWebEngineScript;

    /// <p>Constructs a copy of <i>other</i>.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#QList-1">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Constructs a copy of <i>other</i>.</p>
    /// <p>This operation takes <a href="http://doc.qt.io/qt-5/containers.html#algorithmic-complexity">constant time</a>, because QList is <a href="http://doc.qt.io/qt-5/implicit-sharing.html">implicitly shared</a>. This makes returning a QList from a function very fast. If a shared instance is modified, it will be copied (copy-on-write), and that takes <a href="http://doc.qt.io/qt-5/containers.html#algorithmic-complexity">linear time</a>.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#operator-eq">operator=</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_QList1(
        l: *const crate::QListOfQWebEngineHistoryItem,
    ) -> *mut crate::QListOfQWebEngineHistoryItem;

    /// <p>Constructs a copy of <i>other</i>.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#QList-1">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Constructs a copy of <i>other</i>.</p>
    /// <p>This operation takes <a href="http://doc.qt.io/qt-5/containers.html#algorithmic-complexity">constant time</a>, because QList is <a href="http://doc.qt.io/qt-5/implicit-sharing.html">implicitly shared</a>. This makes returning a QList from a function very fast. If a shared instance is modified, it will be copied (copy-on-write), and that takes <a href="http://doc.qt.io/qt-5/containers.html#algorithmic-complexity">linear time</a>.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#operator-eq">operator=</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_QList1(
        l: *const crate::QListOfQWebEngineScript,
    ) -> *mut crate::QListOfQWebEngineScript;

    /// <p>Destroys the list. References to the values in the list and all iterators of this list become invalid.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#dtor.QList">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Destroys the list. References to the values in the list and all iterators of this list become invalid.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_dQList(
        this_ptr: *mut crate::QListOfQWebEngineHistoryItem,
    );

    /// <p>Destroys the list. References to the values in the list and all iterators of this list become invalid.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#dtor.QList">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Destroys the list. References to the values in the list and all iterators of this list become invalid.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_dQList(
        this_ptr: *mut crate::QListOfQWebEngineScript,
    );

    /// <p>Assigns <i>other</i> to this list and returns a reference to this list.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#operator-eq">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Assigns <i>other</i> to this list and returns a reference to this list.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_operator_(
        this_ptr: *mut crate::QListOfQWebEngineHistoryItem,
        l: *const crate::QListOfQWebEngineHistoryItem,
    ) -> *mut crate::QListOfQWebEngineHistoryItem;

    /// <p>Assigns <i>other</i> to this list and returns a reference to this list.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#operator-eq">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Assigns <i>other</i> to this list and returns a reference to this list.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_operator_(
        this_ptr: *mut crate::QListOfQWebEngineScript,
        l: *const crate::QListOfQWebEngineScript,
    ) -> *mut crate::QListOfQWebEngineScript;

    /// <p>Swaps list <i>other</i> with this list. This operation is very fast and never fails.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#swap">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Swaps list <i>other</i> with this list. This operation is very fast and never fails.</p>
    /// <p>This function was introduced in Qt 4.8.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_swap(
        this_ptr: *mut crate::QListOfQWebEngineHistoryItem,
        other: *mut crate::QListOfQWebEngineHistoryItem,
    );

    /// <p>Swaps list <i>other</i> with this list. This operation is very fast and never fails.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#swap">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Swaps list <i>other</i> with this list. This operation is very fast and never fails.</p>
    /// <p>This function was introduced in Qt 4.8.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_swap(
        this_ptr: *mut crate::QListOfQWebEngineScript,
        other: *mut crate::QListOfQWebEngineScript,
    );

    /// <p>Returns <code>true</code> if <i>other</i> is equal to this list; otherwise returns false.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#operator-eq-eq">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if <i>other</i> is equal to this list; otherwise returns false.</p>
    /// <p>Two lists are considered equal if they contain the same values in the same order.</p>
    /// <p>This function requires the value type to have an implementation of <code>operator==()</code>.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#operator-not-eq">operator!=</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_operator__(
        this_ptr: *const crate::QListOfQWebEngineScript,
        l: *const crate::QListOfQWebEngineScript,
    ) -> bool;

    /// <p>Returns the number of items in the list.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#size">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns the number of items in the list.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#count-1">count</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_size(
        this_ptr: *const crate::QListOfQWebEngineHistoryItem,
    ) -> ::std::os::raw::c_int;

    /// <p>Returns the number of items in the list.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#size">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns the number of items in the list.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#count-1">count</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_size(
        this_ptr: *const crate::QListOfQWebEngineScript,
    ) -> ::std::os::raw::c_int;

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_detach(
        this_ptr: *mut crate::QListOfQWebEngineHistoryItem,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_detach(
        this_ptr: *mut crate::QListOfQWebEngineScript,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_detachShared(
        this_ptr: *mut crate::QListOfQWebEngineHistoryItem,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_detachShared(
        this_ptr: *mut crate::QListOfQWebEngineScript,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_isDetached(
        this_ptr: *const crate::QListOfQWebEngineHistoryItem,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_isDetached(
        this_ptr: *const crate::QListOfQWebEngineScript,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_setSharable(
        this_ptr: *mut crate::QListOfQWebEngineHistoryItem,
        sharable: bool,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_setSharable(
        this_ptr: *mut crate::QListOfQWebEngineScript,
        sharable: bool,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_isSharedWith(
        this_ptr: *const crate::QListOfQWebEngineHistoryItem,
        other: *const crate::QListOfQWebEngineHistoryItem,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_isSharedWith(
        this_ptr: *const crate::QListOfQWebEngineScript,
        other: *const crate::QListOfQWebEngineScript,
    ) -> bool;

    /// <p>Returns <code>true</code> if the list contains no items; otherwise returns false.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if the list contains no items; otherwise returns false.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_isEmpty(
        this_ptr: *const crate::QListOfQWebEngineHistoryItem,
    ) -> bool;

    /// <p>Returns <code>true</code> if the list contains no items; otherwise returns false.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if the list contains no items; otherwise returns false.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_isEmpty(
        this_ptr: *const crate::QListOfQWebEngineScript,
    ) -> bool;

    /// <p>Removes all items from the list.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#clear">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Removes all items from the list.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#removeAll">removeAll</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_clear(
        this_ptr: *mut crate::QListOfQWebEngineHistoryItem,
    );

    /// <p>Removes all items from the list.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#clear">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Removes all items from the list.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#removeAll">removeAll</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_clear(
        this_ptr: *mut crate::QListOfQWebEngineScript,
    );

    /// <p>Returns the item at index position <i>i</i> in the list. <i>i</i> must be a valid index position in the list (i.e., 0 &lt;= <i>i</i> &lt; <a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>()).</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#at">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns the item at index position <i>i</i> in the list. <i>i</i> must be a valid index position in the list (i.e., 0 &lt;= <i>i</i> &lt; <a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>()).</p>
    /// <p>This function is very fast (<a href="http://doc.qt.io/qt-5/containers.html#algorithmic-complexity">constant time</a>).</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#value">value</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#operator-5b-5d">operator[]</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_at(
        this_ptr: *const crate::QListOfQWebEngineHistoryItem,
        i: ::std::os::raw::c_int,
    ) -> *const crate::QWebEngineHistoryItem;

    /// <p>Returns the item at index position <i>i</i> in the list. <i>i</i> must be a valid index position in the list (i.e., 0 &lt;= <i>i</i> &lt; <a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>()).</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#at">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns the item at index position <i>i</i> in the list. <i>i</i> must be a valid index position in the list (i.e., 0 &lt;= <i>i</i> &lt; <a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>()).</p>
    /// <p>This function is very fast (<a href="http://doc.qt.io/qt-5/containers.html#algorithmic-complexity">constant time</a>).</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#value">value</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#operator-5b-5d">operator[]</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_at(
        this_ptr: *const crate::QListOfQWebEngineScript,
        i: ::std::os::raw::c_int,
    ) -> *const crate::QWebEngineScript;

    /// <p>This is an overloaded function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#operator-5b-5d-1">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This is an overloaded function.</p>
    /// <p>Same as <a href="http://doc.qt.io/qt-5/qlist.html#at">at</a>(). This function runs in <a href="http://doc.qt.io/qt-5/containers.html#algorithmic-complexity">constant time</a>.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_operator__2(
        this_ptr: *const crate::QListOfQWebEngineHistoryItem,
        i: ::std::os::raw::c_int,
    ) -> *const crate::QWebEngineHistoryItem;

    /// <p>This is an overloaded function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#operator-5b-5d-1">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This is an overloaded function.</p>
    /// <p>Same as <a href="http://doc.qt.io/qt-5/qlist.html#at">at</a>(). This function runs in <a href="http://doc.qt.io/qt-5/containers.html#algorithmic-complexity">constant time</a>.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_operator__2(
        this_ptr: *const crate::QListOfQWebEngineScript,
        i: ::std::os::raw::c_int,
    ) -> *const crate::QWebEngineScript;

    /// <p>Returns the item at index position <i>i</i> as a modifiable reference. <i>i</i> must be a valid index position in the list (i.e., 0 &lt;= <i>i</i> &lt; <a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>()).</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#operator-5b-5d">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns the item at index position <i>i</i> as a modifiable reference. <i>i</i> must be a valid index position in the list (i.e., 0 &lt;= <i>i</i> &lt; <a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>()).</p>
    /// <p>If this function is called on a list that is currently being shared, it will trigger a copy of all elements. Otherwise, this function runs in <a href="http://doc.qt.io/qt-5/containers.html#algorithmic-complexity">constant time</a>. If you do not want to modify the list you should use <a href="http://doc.qt.io/qt-5/qlist.html#at">QList::at</a>().</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#at">at</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#value">value</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_operator__3(
        this_ptr: *mut crate::QListOfQWebEngineHistoryItem,
        i: ::std::os::raw::c_int,
    ) -> *mut crate::QWebEngineHistoryItem;

    /// <p>Returns the item at index position <i>i</i> as a modifiable reference. <i>i</i> must be a valid index position in the list (i.e., 0 &lt;= <i>i</i> &lt; <a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>()).</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#operator-5b-5d">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns the item at index position <i>i</i> as a modifiable reference. <i>i</i> must be a valid index position in the list (i.e., 0 &lt;= <i>i</i> &lt; <a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>()).</p>
    /// <p>If this function is called on a list that is currently being shared, it will trigger a copy of all elements. Otherwise, this function runs in <a href="http://doc.qt.io/qt-5/containers.html#algorithmic-complexity">constant time</a>. If you do not want to modify the list you should use <a href="http://doc.qt.io/qt-5/qlist.html#at">QList::at</a>().</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#at">at</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#value">value</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_operator__3(
        this_ptr: *mut crate::QListOfQWebEngineScript,
        i: ::std::os::raw::c_int,
    ) -> *mut crate::QWebEngineScript;

    /// <p>Reserve space for <i>alloc</i> elements.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#reserve">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reserve space for <i>alloc</i> elements.</p>
    /// <p>If <i>alloc</i> is smaller than the current size of the list, nothing will happen.</p>
    /// <p>Use this function to avoid repetetive reallocation of <a href="http://doc.qt.io/qt-5/qlist.html">QList</a>'s internal data if you can predict how many elements will be appended. Note that the reservation applies only to the internal pointer array.</p>
    /// <p>This function was introduced in Qt 4.7.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_reserve(
        this_ptr: *mut crate::QListOfQWebEngineHistoryItem,
        size: ::std::os::raw::c_int,
    );

    /// <p>Reserve space for <i>alloc</i> elements.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#reserve">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reserve space for <i>alloc</i> elements.</p>
    /// <p>If <i>alloc</i> is smaller than the current size of the list, nothing will happen.</p>
    /// <p>Use this function to avoid repetetive reallocation of <a href="http://doc.qt.io/qt-5/qlist.html">QList</a>'s internal data if you can predict how many elements will be appended. Note that the reservation applies only to the internal pointer array.</p>
    /// <p>This function was introduced in Qt 4.7.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_reserve(
        this_ptr: *mut crate::QListOfQWebEngineScript,
        size: ::std::os::raw::c_int,
    );

    /// <p>Inserts <i>value</i> at the end of the list.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#append">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Inserts <i>value</i> at the end of the list.</p>
    /// <p>Example:</p>
    /// <pre class="cpp"> <span class="type"><a href="http://doc.qt.io/qt-5/qlist.html#QList">QList</a></span><span class="operator">&lt;</span><span class="type"><a href="http://doc.qt.io/qt-5/qstring.html">QString</a></span><span class="operator">&gt;</span> list;
    ///  list<span class="operator">.</span>append(<span class="string">"one"</span>);
    ///  list<span class="operator">.</span>append(<span class="string">"two"</span>);
    ///  list<span class="operator">.</span>append(<span class="string">"three"</span>);
    ///  <span class="comment">// list: ["one", "two", "three"]</span>
    /// </pre>
    /// <p>This is the same as list.insert(<a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>(), <i>value</i>).</p>
    /// <p>If this list is not shared, this operation is typically very fast (amortized <a href="http://doc.qt.io/qt-5/containers.html#algorithmic-complexity">constant time</a>), because <a href="http://doc.qt.io/qt-5/qlist.html">QList</a> preallocates extra space on both sides of its internal buffer to allow for fast growth at both ends of the list.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#operator-lt-lt">operator&lt;&lt;</a>(), <a href="http://doc.qt.io/qt-5/qlist.html#prepend">prepend</a>(), and <a href="http://doc.qt.io/qt-5/qlist.html#insert">insert</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_append(
        this_ptr: *mut crate::QListOfQWebEngineHistoryItem,
        t: *const crate::QWebEngineHistoryItem,
    );

    /// <p>Inserts <i>value</i> at the end of the list.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#append">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Inserts <i>value</i> at the end of the list.</p>
    /// <p>Example:</p>
    /// <pre class="cpp"> <span class="type"><a href="http://doc.qt.io/qt-5/qlist.html#QList">QList</a></span><span class="operator">&lt;</span><span class="type"><a href="http://doc.qt.io/qt-5/qstring.html">QString</a></span><span class="operator">&gt;</span> list;
    ///  list<span class="operator">.</span>append(<span class="string">"one"</span>);
    ///  list<span class="operator">.</span>append(<span class="string">"two"</span>);
    ///  list<span class="operator">.</span>append(<span class="string">"three"</span>);
    ///  <span class="comment">// list: ["one", "two", "three"]</span>
    /// </pre>
    /// <p>This is the same as list.insert(<a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>(), <i>value</i>).</p>
    /// <p>If this list is not shared, this operation is typically very fast (amortized <a href="http://doc.qt.io/qt-5/containers.html#algorithmic-complexity">constant time</a>), because <a href="http://doc.qt.io/qt-5/qlist.html">QList</a> preallocates extra space on both sides of its internal buffer to allow for fast growth at both ends of the list.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#operator-lt-lt">operator&lt;&lt;</a>(), <a href="http://doc.qt.io/qt-5/qlist.html#prepend">prepend</a>(), and <a href="http://doc.qt.io/qt-5/qlist.html#insert">insert</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_append(
        this_ptr: *mut crate::QListOfQWebEngineScript,
        t: *const crate::QWebEngineScript,
    );

    /// <p>This is an overloaded function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#append-1">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This is an overloaded function.</p>
    /// <p>Appends the items of the <i>value</i> list to this list.</p>
    /// <p>This function was introduced in Qt 4.5.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#operator-lt-lt">operator&lt;&lt;</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#operator-2b-eq">operator+=</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_append1(
        this_ptr: *mut crate::QListOfQWebEngineHistoryItem,
        t: *const crate::QListOfQWebEngineHistoryItem,
    );

    /// <p>This is an overloaded function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#append-1">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This is an overloaded function.</p>
    /// <p>Appends the items of the <i>value</i> list to this list.</p>
    /// <p>This function was introduced in Qt 4.5.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#operator-lt-lt">operator&lt;&lt;</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#operator-2b-eq">operator+=</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_append1(
        this_ptr: *mut crate::QListOfQWebEngineScript,
        t: *const crate::QListOfQWebEngineScript,
    );

    /// <p>Inserts <i>value</i> at the beginning of the list.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#prepend">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Inserts <i>value</i> at the beginning of the list.</p>
    /// <p>Example:</p>
    /// <pre class="cpp"> <span class="type"><a href="http://doc.qt.io/qt-5/qlist.html#QList">QList</a></span><span class="operator">&lt;</span><span class="type"><a href="http://doc.qt.io/qt-5/qstring.html">QString</a></span><span class="operator">&gt;</span> list;
    ///  list<span class="operator">.</span>prepend(<span class="string">"one"</span>);
    ///  list<span class="operator">.</span>prepend(<span class="string">"two"</span>);
    ///  list<span class="operator">.</span>prepend(<span class="string">"three"</span>);
    ///  <span class="comment">// list: ["three", "two", "one"]</span>
    /// </pre>
    /// <p>This is the same as list.insert(0, <i>value</i>).</p>
    /// <p>If this list is not shared, this operation is typically very fast (amortized <a href="http://doc.qt.io/qt-5/containers.html#algorithmic-complexity">constant time</a>), because <a href="http://doc.qt.io/qt-5/qlist.html">QList</a> preallocates extra space on both sides of its internal buffer to allow for fast growth at both ends of the list.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#append">append</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#insert">insert</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_prepend(
        this_ptr: *mut crate::QListOfQWebEngineHistoryItem,
        t: *const crate::QWebEngineHistoryItem,
    );

    /// <p>Inserts <i>value</i> at the beginning of the list.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#prepend">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Inserts <i>value</i> at the beginning of the list.</p>
    /// <p>Example:</p>
    /// <pre class="cpp"> <span class="type"><a href="http://doc.qt.io/qt-5/qlist.html#QList">QList</a></span><span class="operator">&lt;</span><span class="type"><a href="http://doc.qt.io/qt-5/qstring.html">QString</a></span><span class="operator">&gt;</span> list;
    ///  list<span class="operator">.</span>prepend(<span class="string">"one"</span>);
    ///  list<span class="operator">.</span>prepend(<span class="string">"two"</span>);
    ///  list<span class="operator">.</span>prepend(<span class="string">"three"</span>);
    ///  <span class="comment">// list: ["three", "two", "one"]</span>
    /// </pre>
    /// <p>This is the same as list.insert(0, <i>value</i>).</p>
    /// <p>If this list is not shared, this operation is typically very fast (amortized <a href="http://doc.qt.io/qt-5/containers.html#algorithmic-complexity">constant time</a>), because <a href="http://doc.qt.io/qt-5/qlist.html">QList</a> preallocates extra space on both sides of its internal buffer to allow for fast growth at both ends of the list.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#append">append</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#insert">insert</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_prepend(
        this_ptr: *mut crate::QListOfQWebEngineScript,
        t: *const crate::QWebEngineScript,
    );

    /// <p>Inserts <i>value</i> at index position <i>i</i> in the list.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#insert">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Inserts <i>value</i> at index position <i>i</i> in the list.</p>
    /// <p>If <i>i</i> == 0, the value is prepended to the list. If <i>i</i> == <a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>(), the value is appended to the list.</p>
    /// <p>Example:</p>
    /// <pre class="cpp"> <span class="type"><a href="http://doc.qt.io/qt-5/qlist.html#QList">QList</a></span><span class="operator">&lt;</span><span class="type"><a href="http://doc.qt.io/qt-5/qstring.html">QString</a></span><span class="operator">&gt;</span> list;
    ///  list <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"alpha"</span> <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"beta"</span> <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"delta"</span>;
    ///  list<span class="operator">.</span>insert(<span class="number">2</span><span class="operator">,</span> <span class="string">"gamma"</span>);
    ///  <span class="comment">// list: ["alpha", "beta", "gamma", "delta"]</span>
    /// </pre>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#append">append</a>(), <a href="http://doc.qt.io/qt-5/qlist.html#prepend">prepend</a>(), <a href="http://doc.qt.io/qt-5/qlist.html#replace">replace</a>(), and <a href="http://doc.qt.io/qt-5/qlist.html#removeAt">removeAt</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_insert(
        this_ptr: *mut crate::QListOfQWebEngineHistoryItem,
        i: ::std::os::raw::c_int,
        t: *const crate::QWebEngineHistoryItem,
    );

    /// <p>Inserts <i>value</i> at index position <i>i</i> in the list.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#insert">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Inserts <i>value</i> at index position <i>i</i> in the list.</p>
    /// <p>If <i>i</i> == 0, the value is prepended to the list. If <i>i</i> == <a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>(), the value is appended to the list.</p>
    /// <p>Example:</p>
    /// <pre class="cpp"> <span class="type"><a href="http://doc.qt.io/qt-5/qlist.html#QList">QList</a></span><span class="operator">&lt;</span><span class="type"><a href="http://doc.qt.io/qt-5/qstring.html">QString</a></span><span class="operator">&gt;</span> list;
    ///  list <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"alpha"</span> <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"beta"</span> <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"delta"</span>;
    ///  list<span class="operator">.</span>insert(<span class="number">2</span><span class="operator">,</span> <span class="string">"gamma"</span>);
    ///  <span class="comment">// list: ["alpha", "beta", "gamma", "delta"]</span>
    /// </pre>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#append">append</a>(), <a href="http://doc.qt.io/qt-5/qlist.html#prepend">prepend</a>(), <a href="http://doc.qt.io/qt-5/qlist.html#replace">replace</a>(), and <a href="http://doc.qt.io/qt-5/qlist.html#removeAt">removeAt</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_insert(
        this_ptr: *mut crate::QListOfQWebEngineScript,
        i: ::std::os::raw::c_int,
        t: *const crate::QWebEngineScript,
    );

    /// <p>Replaces the item at index position <i>i</i> with <i>value</i>. <i>i</i> must be a valid index position in the list (i.e., 0 &lt;= <i>i</i> &lt; <a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>()).</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#replace">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Replaces the item at index position <i>i</i> with <i>value</i>. <i>i</i> must be a valid index position in the list (i.e., 0 &lt;= <i>i</i> &lt; <a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>()).</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#operator-5b-5d">operator[]</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#removeAt">removeAt</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_replace(
        this_ptr: *mut crate::QListOfQWebEngineHistoryItem,
        i: ::std::os::raw::c_int,
        t: *const crate::QWebEngineHistoryItem,
    );

    /// <p>Replaces the item at index position <i>i</i> with <i>value</i>. <i>i</i> must be a valid index position in the list (i.e., 0 &lt;= <i>i</i> &lt; <a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>()).</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#replace">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Replaces the item at index position <i>i</i> with <i>value</i>. <i>i</i> must be a valid index position in the list (i.e., 0 &lt;= <i>i</i> &lt; <a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>()).</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#operator-5b-5d">operator[]</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#removeAt">removeAt</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_replace(
        this_ptr: *mut crate::QListOfQWebEngineScript,
        i: ::std::os::raw::c_int,
        t: *const crate::QWebEngineScript,
    );

    /// <p>Removes the item at index position <i>i</i>. <i>i</i> must be a valid index position in the list (i.e., 0 &lt;= <i>i</i> &lt; <a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>()).</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#removeAt">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Removes the item at index position <i>i</i>. <i>i</i> must be a valid index position in the list (i.e., 0 &lt;= <i>i</i> &lt; <a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>()).</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#takeAt">takeAt</a>(), <a href="http://doc.qt.io/qt-5/qlist.html#removeFirst">removeFirst</a>(), <a href="http://doc.qt.io/qt-5/qlist.html#removeLast">removeLast</a>(), and <a href="http://doc.qt.io/qt-5/qlist.html#removeOne">removeOne</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_removeAt(
        this_ptr: *mut crate::QListOfQWebEngineHistoryItem,
        i: ::std::os::raw::c_int,
    );

    /// <p>Removes the item at index position <i>i</i>. <i>i</i> must be a valid index position in the list (i.e., 0 &lt;= <i>i</i> &lt; <a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>()).</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#removeAt">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Removes the item at index position <i>i</i>. <i>i</i> must be a valid index position in the list (i.e., 0 &lt;= <i>i</i> &lt; <a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>()).</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#takeAt">takeAt</a>(), <a href="http://doc.qt.io/qt-5/qlist.html#removeFirst">removeFirst</a>(), <a href="http://doc.qt.io/qt-5/qlist.html#removeLast">removeLast</a>(), and <a href="http://doc.qt.io/qt-5/qlist.html#removeOne">removeOne</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_removeAt(
        this_ptr: *mut crate::QListOfQWebEngineScript,
        i: ::std::os::raw::c_int,
    );

    /// <p>Removes all occurrences of <i>value</i> in the list and returns the number of entries removed.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#removeAll">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Removes all occurrences of <i>value</i> in the list and returns the number of entries removed.</p>
    /// <p>Example:</p>
    /// <pre class="cpp"> <span class="type"><a href="http://doc.qt.io/qt-5/qlist.html#QList">QList</a></span><span class="operator">&lt;</span><span class="type"><a href="http://doc.qt.io/qt-5/qstring.html">QString</a></span><span class="operator">&gt;</span> list;
    ///  list <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"sun"</span> <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"cloud"</span> <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"sun"</span> <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"rain"</span>;
    ///  list<span class="operator">.</span>removeAll(<span class="string">"sun"</span>);
    ///  <span class="comment">// list: ["cloud", "rain"]</span>
    /// </pre>
    /// <p>This function requires the value type to have an implementation of <code>operator==()</code>.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#removeOne">removeOne</a>(), <a href="http://doc.qt.io/qt-5/qlist.html#removeAt">removeAt</a>(), <a href="http://doc.qt.io/qt-5/qlist.html#takeAt">takeAt</a>(), and <a href="http://doc.qt.io/qt-5/qlist.html#replace">replace</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_removeAll(
        this_ptr: *mut crate::QListOfQWebEngineScript,
        t: *const crate::QWebEngineScript,
    ) -> ::std::os::raw::c_int;

    /// <p>Removes the first occurrence of <i>value</i> in the list and returns true on success; otherwise returns <code>false</code>.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#removeOne">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Removes the first occurrence of <i>value</i> in the list and returns true on success; otherwise returns <code>false</code>.</p>
    /// <p>Example:</p>
    /// <pre class="cpp"> <span class="type"><a href="http://doc.qt.io/qt-5/qlist.html#QList">QList</a></span><span class="operator">&lt;</span><span class="type"><a href="http://doc.qt.io/qt-5/qstring.html">QString</a></span><span class="operator">&gt;</span> list;
    ///  list <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"sun"</span> <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"cloud"</span> <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"sun"</span> <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"rain"</span>;
    ///  list<span class="operator">.</span>removeOne(<span class="string">"sun"</span>);
    ///  <span class="comment">// list: ["cloud", "sun", "rain"]</span>
    /// </pre>
    /// <p>This function requires the value type to have an implementation of <code>operator==()</code>.</p>
    /// <p>This function was introduced in Qt 4.4.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#removeAll">removeAll</a>(), <a href="http://doc.qt.io/qt-5/qlist.html#removeAt">removeAt</a>(), <a href="http://doc.qt.io/qt-5/qlist.html#takeAt">takeAt</a>(), and <a href="http://doc.qt.io/qt-5/qlist.html#replace">replace</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_removeOne(
        this_ptr: *mut crate::QListOfQWebEngineScript,
        t: *const crate::QWebEngineScript,
    ) -> bool;

    /// <p>Removes the item at index position <i>i</i> and returns it. <i>i</i> must be a valid index position in the list (i.e., 0 &lt;= <i>i</i> &lt; <a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>()).</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#takeAt">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Removes the item at index position <i>i</i> and returns it. <i>i</i> must be a valid index position in the list (i.e., 0 &lt;= <i>i</i> &lt; <a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>()).</p>
    /// <p>If you don't use the return value, <a href="http://doc.qt.io/qt-5/qlist.html#removeAt">removeAt</a>() is more efficient.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#removeAt">removeAt</a>(), <a href="http://doc.qt.io/qt-5/qlist.html#takeFirst">takeFirst</a>(), and <a href="http://doc.qt.io/qt-5/qlist.html#takeLast">takeLast</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_takeAt(
        this_ptr: *mut crate::QListOfQWebEngineHistoryItem,
        i: ::std::os::raw::c_int,
    ) -> *mut crate::QWebEngineHistoryItem;

    /// <p>Removes the item at index position <i>i</i> and returns it. <i>i</i> must be a valid index position in the list (i.e., 0 &lt;= <i>i</i> &lt; <a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>()).</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#takeAt">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Removes the item at index position <i>i</i> and returns it. <i>i</i> must be a valid index position in the list (i.e., 0 &lt;= <i>i</i> &lt; <a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>()).</p>
    /// <p>If you don't use the return value, <a href="http://doc.qt.io/qt-5/qlist.html#removeAt">removeAt</a>() is more efficient.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#removeAt">removeAt</a>(), <a href="http://doc.qt.io/qt-5/qlist.html#takeFirst">takeFirst</a>(), and <a href="http://doc.qt.io/qt-5/qlist.html#takeLast">takeLast</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_takeAt(
        this_ptr: *mut crate::QListOfQWebEngineScript,
        i: ::std::os::raw::c_int,
    ) -> *mut crate::QWebEngineScript;

    /// <p>Removes the first item in the list and returns it. This is the same as <a href="http://doc.qt.io/qt-5/qlist.html#takeAt">takeAt</a>(0). This function assumes the list is not empty. To avoid failure, call <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() before calling this function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#takeFirst">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Removes the first item in the list and returns it. This is the same as <a href="http://doc.qt.io/qt-5/qlist.html#takeAt">takeAt</a>(0). This function assumes the list is not empty. To avoid failure, call <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() before calling this function.</p>
    /// <p>If this list is not shared, this operation takes <a href="http://doc.qt.io/qt-5/containers.html#algorithmic-complexity">constant time</a>.</p>
    /// <p>If you don't use the return value, <a href="http://doc.qt.io/qt-5/qlist.html#removeFirst">removeFirst</a>() is more efficient.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#takeLast">takeLast</a>(), <a href="http://doc.qt.io/qt-5/qlist.html#takeAt">takeAt</a>(), and <a href="http://doc.qt.io/qt-5/qlist.html#removeFirst">removeFirst</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_takeFirst(
        this_ptr: *mut crate::QListOfQWebEngineHistoryItem,
    ) -> *mut crate::QWebEngineHistoryItem;

    /// <p>Removes the first item in the list and returns it. This is the same as <a href="http://doc.qt.io/qt-5/qlist.html#takeAt">takeAt</a>(0). This function assumes the list is not empty. To avoid failure, call <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() before calling this function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#takeFirst">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Removes the first item in the list and returns it. This is the same as <a href="http://doc.qt.io/qt-5/qlist.html#takeAt">takeAt</a>(0). This function assumes the list is not empty. To avoid failure, call <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() before calling this function.</p>
    /// <p>If this list is not shared, this operation takes <a href="http://doc.qt.io/qt-5/containers.html#algorithmic-complexity">constant time</a>.</p>
    /// <p>If you don't use the return value, <a href="http://doc.qt.io/qt-5/qlist.html#removeFirst">removeFirst</a>() is more efficient.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#takeLast">takeLast</a>(), <a href="http://doc.qt.io/qt-5/qlist.html#takeAt">takeAt</a>(), and <a href="http://doc.qt.io/qt-5/qlist.html#removeFirst">removeFirst</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_takeFirst(
        this_ptr: *mut crate::QListOfQWebEngineScript,
    ) -> *mut crate::QWebEngineScript;

    /// <p>Removes the last item in the list and returns it. This is the same as <a href="http://doc.qt.io/qt-5/qlist.html#takeAt">takeAt</a>(<a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>() - 1). This function assumes the list is not empty. To avoid failure, call <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() before calling this function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#takeLast">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Removes the last item in the list and returns it. This is the same as <a href="http://doc.qt.io/qt-5/qlist.html#takeAt">takeAt</a>(<a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>() - 1). This function assumes the list is not empty. To avoid failure, call <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() before calling this function.</p>
    /// <p>If this list is not shared, this operation takes <a href="http://doc.qt.io/qt-5/containers.html#algorithmic-complexity">constant time</a>.</p>
    /// <p>If you don't use the return value, <a href="http://doc.qt.io/qt-5/qlist.html#removeLast">removeLast</a>() is more efficient.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#takeFirst">takeFirst</a>(), <a href="http://doc.qt.io/qt-5/qlist.html#takeAt">takeAt</a>(), and <a href="http://doc.qt.io/qt-5/qlist.html#removeLast">removeLast</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_takeLast(
        this_ptr: *mut crate::QListOfQWebEngineHistoryItem,
    ) -> *mut crate::QWebEngineHistoryItem;

    /// <p>Removes the last item in the list and returns it. This is the same as <a href="http://doc.qt.io/qt-5/qlist.html#takeAt">takeAt</a>(<a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>() - 1). This function assumes the list is not empty. To avoid failure, call <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() before calling this function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#takeLast">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Removes the last item in the list and returns it. This is the same as <a href="http://doc.qt.io/qt-5/qlist.html#takeAt">takeAt</a>(<a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>() - 1). This function assumes the list is not empty. To avoid failure, call <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() before calling this function.</p>
    /// <p>If this list is not shared, this operation takes <a href="http://doc.qt.io/qt-5/containers.html#algorithmic-complexity">constant time</a>.</p>
    /// <p>If you don't use the return value, <a href="http://doc.qt.io/qt-5/qlist.html#removeLast">removeLast</a>() is more efficient.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#takeFirst">takeFirst</a>(), <a href="http://doc.qt.io/qt-5/qlist.html#takeAt">takeAt</a>(), and <a href="http://doc.qt.io/qt-5/qlist.html#removeLast">removeLast</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_takeLast(
        this_ptr: *mut crate::QListOfQWebEngineScript,
    ) -> *mut crate::QWebEngineScript;

    /// <p>Moves the item at index position <i>from</i> to index position <i>to</i>.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#move">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Moves the item at index position <i>from</i> to index position <i>to</i>.</p>
    /// <p>Example:</p>
    /// <pre class="cpp"> <span class="type"><a href="http://doc.qt.io/qt-5/qlist.html#QList">QList</a></span><span class="operator">&lt;</span><span class="type"><a href="http://doc.qt.io/qt-5/qstring.html">QString</a></span><span class="operator">&gt;</span> list;
    ///  list <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"A"</span> <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"B"</span> <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"C"</span> <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"D"</span> <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"E"</span> <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"F"</span>;
    ///  list<span class="operator">.</span>move(<span class="number">1</span><span class="operator">,</span> <span class="number">4</span>);
    ///  <span class="comment">// list: ["A", "C", "D", "E", "B", "F"]</span>
    /// </pre>
    /// <p>This is the same as insert(<i>to</i>, <a href="http://doc.qt.io/qt-5/qlist.html#takeAt">takeAt</a>(<i>from</i>)).This function assumes that both <i>from</i> and <i>to</i> are at least 0 but less than <a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>(). To avoid failure, test that both <i>from</i> and <i>to</i> are at least 0 and less than <a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>().</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#swap">swap</a>(), <a href="http://doc.qt.io/qt-5/qlist.html#insert">insert</a>(), and <a href="http://doc.qt.io/qt-5/qlist.html#takeAt">takeAt</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_move(
        this_ptr: *mut crate::QListOfQWebEngineHistoryItem,
        from: ::std::os::raw::c_int,
        to: ::std::os::raw::c_int,
    );

    /// <p>Moves the item at index position <i>from</i> to index position <i>to</i>.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#move">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Moves the item at index position <i>from</i> to index position <i>to</i>.</p>
    /// <p>Example:</p>
    /// <pre class="cpp"> <span class="type"><a href="http://doc.qt.io/qt-5/qlist.html#QList">QList</a></span><span class="operator">&lt;</span><span class="type"><a href="http://doc.qt.io/qt-5/qstring.html">QString</a></span><span class="operator">&gt;</span> list;
    ///  list <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"A"</span> <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"B"</span> <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"C"</span> <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"D"</span> <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"E"</span> <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"F"</span>;
    ///  list<span class="operator">.</span>move(<span class="number">1</span><span class="operator">,</span> <span class="number">4</span>);
    ///  <span class="comment">// list: ["A", "C", "D", "E", "B", "F"]</span>
    /// </pre>
    /// <p>This is the same as insert(<i>to</i>, <a href="http://doc.qt.io/qt-5/qlist.html#takeAt">takeAt</a>(<i>from</i>)).This function assumes that both <i>from</i> and <i>to</i> are at least 0 but less than <a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>(). To avoid failure, test that both <i>from</i> and <i>to</i> are at least 0 and less than <a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>().</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#swap">swap</a>(), <a href="http://doc.qt.io/qt-5/qlist.html#insert">insert</a>(), and <a href="http://doc.qt.io/qt-5/qlist.html#takeAt">takeAt</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_move(
        this_ptr: *mut crate::QListOfQWebEngineScript,
        from: ::std::os::raw::c_int,
        to: ::std::os::raw::c_int,
    );

    /// <p>Exchange the item at index position <i>i</i> with the item at index position <i>j</i>. This function assumes that both <i>i</i> and <i>j</i> are at least 0 but less than <a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>(). To avoid failure, test that both <i>i</i> and <i>j</i> are at least 0 and less than <a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>().</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#swapItemsAt">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Exchange the item at index position <i>i</i> with the item at index position <i>j</i>. This function assumes that both <i>i</i> and <i>j</i> are at least 0 but less than <a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>(). To avoid failure, test that both <i>i</i> and <i>j</i> are at least 0 and less than <a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>().</p>
    /// <p>Example:</p>
    /// <pre class="cpp"> <span class="type"><a href="http://doc.qt.io/qt-5/qlist.html#QList">QList</a></span><span class="operator">&lt;</span><span class="type"><a href="http://doc.qt.io/qt-5/qstring.html">QString</a></span><span class="operator">&gt;</span> list;
    ///  list <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"A"</span> <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"B"</span> <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"C"</span> <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"D"</span> <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"E"</span> <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"F"</span>;
    ///  list<span class="operator">.</span>swapItemsAt(<span class="number">1</span><span class="operator">,</span> <span class="number">4</span>);
    ///  <span class="comment">// list: ["A", "E", "C", "D", "B", "F"]</span>
    /// </pre>
    /// <p>This function was introduced in Qt 5.13.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#move">move</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_swapItemsAt(
        this_ptr: *mut crate::QListOfQWebEngineHistoryItem,
        i: ::std::os::raw::c_int,
        j: ::std::os::raw::c_int,
    );

    /// <p>Exchange the item at index position <i>i</i> with the item at index position <i>j</i>. This function assumes that both <i>i</i> and <i>j</i> are at least 0 but less than <a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>(). To avoid failure, test that both <i>i</i> and <i>j</i> are at least 0 and less than <a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>().</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#swapItemsAt">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Exchange the item at index position <i>i</i> with the item at index position <i>j</i>. This function assumes that both <i>i</i> and <i>j</i> are at least 0 but less than <a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>(). To avoid failure, test that both <i>i</i> and <i>j</i> are at least 0 and less than <a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>().</p>
    /// <p>Example:</p>
    /// <pre class="cpp"> <span class="type"><a href="http://doc.qt.io/qt-5/qlist.html#QList">QList</a></span><span class="operator">&lt;</span><span class="type"><a href="http://doc.qt.io/qt-5/qstring.html">QString</a></span><span class="operator">&gt;</span> list;
    ///  list <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"A"</span> <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"B"</span> <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"C"</span> <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"D"</span> <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"E"</span> <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"F"</span>;
    ///  list<span class="operator">.</span>swapItemsAt(<span class="number">1</span><span class="operator">,</span> <span class="number">4</span>);
    ///  <span class="comment">// list: ["A", "E", "C", "D", "B", "F"]</span>
    /// </pre>
    /// <p>This function was introduced in Qt 5.13.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#move">move</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_swapItemsAt(
        this_ptr: *mut crate::QListOfQWebEngineScript,
        i: ::std::os::raw::c_int,
        j: ::std::os::raw::c_int,
    );

    /// <p>Swaps list <i>other</i> with this list. This operation is very fast and never fails.</p>
    ///
    /// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qlist.html#swap">C++ documentation</a> for <span style='color: green;'>```void QList::swap(QList<T> &other)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Swaps list <i>other</i> with this list. This operation is very fast and never fails.</p>
    /// <p>This function was introduced in Qt 4.8.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_swap1(
        this_ptr: *mut crate::QListOfQWebEngineHistoryItem,
        i: ::std::os::raw::c_int,
        j: ::std::os::raw::c_int,
    );

    /// <p>Swaps list <i>other</i> with this list. This operation is very fast and never fails.</p>
    ///
    /// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qlist.html#swap">C++ documentation</a> for <span style='color: green;'>```void QList::swap(QList<T> &other)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Swaps list <i>other</i> with this list. This operation is very fast and never fails.</p>
    /// <p>This function was introduced in Qt 4.8.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_swap1(
        this_ptr: *mut crate::QListOfQWebEngineScript,
        i: ::std::os::raw::c_int,
        j: ::std::os::raw::c_int,
    );

    /// <p>Returns the index position of the first occurrence of <i>value</i> in the list, searching forward from index position <i>from</i>. Returns -1 if no item matched.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#indexOf">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns the index position of the first occurrence of <i>value</i> in the list, searching forward from index position <i>from</i>. Returns -1 if no item matched.</p>
    /// <p>Example:</p>
    /// <pre class="cpp"> <span class="type"><a href="http://doc.qt.io/qt-5/qlist.html#QList">QList</a></span><span class="operator">&lt;</span><span class="type"><a href="http://doc.qt.io/qt-5/qstring.html">QString</a></span><span class="operator">&gt;</span> list;
    ///  list <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"A"</span> <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"B"</span> <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"C"</span> <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"B"</span> <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"A"</span>;
    ///  list<span class="operator">.</span>indexOf(<span class="string">"B"</span>);          <span class="comment">// returns 1</span>
    ///  list<span class="operator">.</span>indexOf(<span class="string">"B"</span><span class="operator">,</span> <span class="number">1</span>);       <span class="comment">// returns 1</span>
    ///  list<span class="operator">.</span>indexOf(<span class="string">"B"</span><span class="operator">,</span> <span class="number">2</span>);       <span class="comment">// returns 3</span>
    ///  list<span class="operator">.</span>indexOf(<span class="string">"X"</span>);          <span class="comment">// returns -1</span>
    /// </pre>
    /// <p>This function requires the value type to have an implementation of <code>operator==()</code>.</p>
    /// <p>Note that <a href="http://doc.qt.io/qt-5/qlist.html">QList</a> uses 0-based indexes, just like C++ arrays. Negative indexes are not supported with the exception of the value mentioned above.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#lastIndexOf">lastIndexOf</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#contains">contains</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_indexOf(
        this_ptr: *const crate::QListOfQWebEngineScript,
        t: *const crate::QWebEngineScript,
        from: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    /// <p>Returns the index position of the last occurrence of <i>value</i> in the list, searching backward from index position <i>from</i>. If <i>from</i> is -1 (the default), the search starts at the last item. Returns -1 if no item matched.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#lastIndexOf">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns the index position of the last occurrence of <i>value</i> in the list, searching backward from index position <i>from</i>. If <i>from</i> is -1 (the default), the search starts at the last item. Returns -1 if no item matched.</p>
    /// <p>Example:</p>
    /// <pre class="cpp"> <span class="type"><a href="http://doc.qt.io/qt-5/qlist.html#QList">QList</a></span><span class="operator">&lt;</span><span class="type"><a href="http://doc.qt.io/qt-5/qstring.html">QString</a></span><span class="operator">&gt;</span> list;
    ///  list <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"A"</span> <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"B"</span> <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"C"</span> <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"B"</span> <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"A"</span>;
    ///  list<span class="operator">.</span>lastIndexOf(<span class="string">"B"</span>);      <span class="comment">// returns 3</span>
    ///  list<span class="operator">.</span>lastIndexOf(<span class="string">"B"</span><span class="operator">,</span> <span class="number">3</span>);   <span class="comment">// returns 3</span>
    ///  list<span class="operator">.</span>lastIndexOf(<span class="string">"B"</span><span class="operator">,</span> <span class="number">2</span>);   <span class="comment">// returns 1</span>
    ///  list<span class="operator">.</span>lastIndexOf(<span class="string">"X"</span>);      <span class="comment">// returns -1</span>
    /// </pre>
    /// <p>This function requires the value type to have an implementation of <code>operator==()</code>.</p>
    /// <p>Note that <a href="http://doc.qt.io/qt-5/qlist.html">QList</a> uses 0-based indexes, just like C++ arrays. Negative indexes are not supported with the exception of the value mentioned above.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#indexOf">indexOf</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_lastIndexOf(
        this_ptr: *const crate::QListOfQWebEngineScript,
        t: *const crate::QWebEngineScript,
        from: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    /// <p>Returns <code>true</code> if the list contains an occurrence of <i>value</i>; otherwise returns <code>false</code>.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#contains">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if the list contains an occurrence of <i>value</i>; otherwise returns <code>false</code>.</p>
    /// <p>This function requires the value type to have an implementation of <code>operator==()</code>.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#indexOf">indexOf</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#count-1">count</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_contains(
        this_ptr: *const crate::QListOfQWebEngineScript,
        t: *const crate::QWebEngineScript,
    ) -> bool;

    /// <p>Returns the number of occurrences of <i>value</i> in the list.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#count">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns the number of occurrences of <i>value</i> in the list.</p>
    /// <p>This function requires the value type to have an implementation of <code>operator==()</code>.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#contains">contains</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#indexOf">indexOf</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_count(
        this_ptr: *const crate::QListOfQWebEngineScript,
        t: *const crate::QWebEngineScript,
    ) -> ::std::os::raw::c_int;

    /// <p>Constructs an uninitialized iterator.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist-iterator.html#iterator">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Constructs an uninitialized iterator.</p>
    /// <p>Functions like operator*() and operator++() should not be called on an uninitialized iterator. Use operator=() to assign a value to it before using it.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#begin">QList::begin</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#end">QList::end</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_iterator_iterator(
    ) -> *mut crate::q_list_of_q_web_engine_history_item::Iterator;

    /// <p>Constructs an uninitialized iterator.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist-iterator.html#iterator">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Constructs an uninitialized iterator.</p>
    /// <p>Functions like operator*() and operator++() should not be called on an uninitialized iterator. Use operator=() to assign a value to it before using it.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#begin">QList::begin</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#end">QList::end</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_iterator_iterator(
    ) -> *mut crate::q_list_of_q_web_engine_script::Iterator;

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_iterator_iterator1(
        o: *const crate::q_list_of_q_web_engine_history_item::Iterator,
    ) -> *mut crate::q_list_of_q_web_engine_history_item::Iterator;

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_iterator_iterator1(
        o: *const crate::q_list_of_q_web_engine_script::Iterator,
    ) -> *mut crate::q_list_of_q_web_engine_script::Iterator;

    /// <p>Assigns <i>other</i> to this iterator.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlinkedlist-iterator.html#operator-eq">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Assigns <i>other</i> to this iterator.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_iterator_operator_(
        this_ptr: *mut crate::q_list_of_q_web_engine_history_item::Iterator,
        o: *const crate::q_list_of_q_web_engine_history_item::Iterator,
    ) -> *mut crate::q_list_of_q_web_engine_history_item::Iterator;

    /// <p>Assigns <i>other</i> to this iterator.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlinkedlist-iterator.html#operator-eq">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Assigns <i>other</i> to this iterator.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_iterator_operator_(
        this_ptr: *mut crate::q_list_of_q_web_engine_script::Iterator,
        o: *const crate::q_list_of_q_web_engine_script::Iterator,
    ) -> *mut crate::q_list_of_q_web_engine_script::Iterator;

    /// <p>Returns a modifiable reference to the current item's value.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qhash-iterator.html#operator-2a">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns a modifiable reference to the current item's value.</p>
    /// <p>Same as <a href="http://doc.qt.io/qt-5/qhash-iterator.html#value">value</a>().</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qhash-iterator.html#key">key</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_iterator_operator_1(
        this_ptr: *const crate::q_list_of_q_web_engine_history_item::Iterator,
    ) -> *mut crate::QWebEngineHistoryItem;

    /// <p>Returns a modifiable reference to the current item's value.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qhash-iterator.html#operator-2a">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns a modifiable reference to the current item's value.</p>
    /// <p>Same as <a href="http://doc.qt.io/qt-5/qhash-iterator.html#value">value</a>().</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qhash-iterator.html#key">key</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_iterator_operator_1(
        this_ptr: *const crate::q_list_of_q_web_engine_script::Iterator,
    ) -> *mut crate::QWebEngineScript;

    /// <p>Returns a pointer to the current item's value.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qhash-iterator.html#operator--gt">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns a pointer to the current item's value.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qhash-iterator.html#value">value</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_iterator_operator__(
        this_ptr: *const crate::q_list_of_q_web_engine_history_item::Iterator,
    ) -> *mut crate::QWebEngineHistoryItem;

    /// <p>Returns a pointer to the current item's value.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qhash-iterator.html#operator--gt">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns a pointer to the current item's value.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qhash-iterator.html#value">value</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_iterator_operator__(
        this_ptr: *const crate::q_list_of_q_web_engine_script::Iterator,
    ) -> *mut crate::QWebEngineScript;

    /// <p>Returns a modifiable reference to the item at offset <i>j</i> from the item pointed to by this iterator (the item at position <code>*this + j</code>).</p>
    ///
    /// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qjsonarray-iterator.html#operator-5b-5d">C++ documentation</a> for <span style='color: green;'>```QJsonValueRef iterator::operator[](int j) const```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns a modifiable reference to the item at offset <i>j</i> from the item pointed to by this iterator (the item at position <code>*this + j</code>).</p>
    /// <p>This function is provided to make <a href="http://doc.qt.io/qt-5/qjsonarray.html">QJsonArray</a> iterators behave like C++ pointers.</p>
    /// <p>The return value is of type <a href="http://doc.qt.io/qt-5/qjsonvalue.html#qjsonvalueref">QJsonValueRef</a>, a helper class for <a href="http://doc.qt.io/qt-5/qjsonarray.html">QJsonArray</a> and <a href="http://doc.qt.io/qt-5/qjsonobject.html">QJsonObject</a>. When you get an object of type <a href="http://doc.qt.io/qt-5/qjsonvalue.html#qjsonvalueref">QJsonValueRef</a>, you can use it as if it were a reference to a <a href="http://doc.qt.io/qt-5/qjsonvalue.html">QJsonValue</a>. If you assign to it, the assignment will apply to the element in the <a href="http://doc.qt.io/qt-5/qjsonarray.html">QJsonArray</a> or <a href="http://doc.qt.io/qt-5/qjsonobject.html">QJsonObject</a> from which you got the reference.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qjsonarray-iterator.html#operator-2b">operator+</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_iterator_operator__1(
        this_ptr: *const crate::q_list_of_q_web_engine_history_item::Iterator,
        j: ::std::os::raw::c_longlong,
    ) -> *mut crate::QWebEngineHistoryItem;

    /// <p>Returns a modifiable reference to the item at offset <i>j</i> from the item pointed to by this iterator (the item at position <code>*this + j</code>).</p>
    ///
    /// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qjsonarray-iterator.html#operator-5b-5d">C++ documentation</a> for <span style='color: green;'>```QJsonValueRef iterator::operator[](int j) const```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns a modifiable reference to the item at offset <i>j</i> from the item pointed to by this iterator (the item at position <code>*this + j</code>).</p>
    /// <p>This function is provided to make <a href="http://doc.qt.io/qt-5/qjsonarray.html">QJsonArray</a> iterators behave like C++ pointers.</p>
    /// <p>The return value is of type <a href="http://doc.qt.io/qt-5/qjsonvalue.html#qjsonvalueref">QJsonValueRef</a>, a helper class for <a href="http://doc.qt.io/qt-5/qjsonarray.html">QJsonArray</a> and <a href="http://doc.qt.io/qt-5/qjsonobject.html">QJsonObject</a>. When you get an object of type <a href="http://doc.qt.io/qt-5/qjsonvalue.html#qjsonvalueref">QJsonValueRef</a>, you can use it as if it were a reference to a <a href="http://doc.qt.io/qt-5/qjsonvalue.html">QJsonValue</a>. If you assign to it, the assignment will apply to the element in the <a href="http://doc.qt.io/qt-5/qjsonarray.html">QJsonArray</a> or <a href="http://doc.qt.io/qt-5/qjsonobject.html">QJsonObject</a> from which you got the reference.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qjsonarray-iterator.html#operator-2b">operator+</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_iterator_operator__1(
        this_ptr: *const crate::q_list_of_q_web_engine_script::Iterator,
        j: ::std::os::raw::c_longlong,
    ) -> *mut crate::QWebEngineScript;

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_iterator_operator__2(
        this_ptr: *const crate::q_list_of_q_web_engine_history_item::Iterator,
        o: *const crate::q_list_of_q_web_engine_history_item::Iterator,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_iterator_operator__2(
        this_ptr: *const crate::q_list_of_q_web_engine_script::Iterator,
        o: *const crate::q_list_of_q_web_engine_script::Iterator,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_iterator_operator_2(
        this_ptr: *const crate::q_list_of_q_web_engine_history_item::Iterator,
        other: *const crate::q_list_of_q_web_engine_history_item::Iterator,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_iterator_operator_2(
        this_ptr: *const crate::q_list_of_q_web_engine_script::Iterator,
        other: *const crate::q_list_of_q_web_engine_script::Iterator,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_iterator_operator__4(
        this_ptr: *const crate::q_list_of_q_web_engine_history_item::Iterator,
        other: *const crate::q_list_of_q_web_engine_history_item::Iterator,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_iterator_operator__4(
        this_ptr: *const crate::q_list_of_q_web_engine_script::Iterator,
        other: *const crate::q_list_of_q_web_engine_script::Iterator,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_iterator_operator_3(
        this_ptr: *const crate::q_list_of_q_web_engine_history_item::Iterator,
        other: *const crate::q_list_of_q_web_engine_history_item::Iterator,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_iterator_operator_3(
        this_ptr: *const crate::q_list_of_q_web_engine_script::Iterator,
        other: *const crate::q_list_of_q_web_engine_script::Iterator,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_iterator_operator__5(
        this_ptr: *const crate::q_list_of_q_web_engine_history_item::Iterator,
        other: *const crate::q_list_of_q_web_engine_history_item::Iterator,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_iterator_operator__5(
        this_ptr: *const crate::q_list_of_q_web_engine_script::Iterator,
        other: *const crate::q_list_of_q_web_engine_script::Iterator,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_iterator_operator__6(
        this_ptr: *const crate::q_list_of_q_web_engine_history_item::Iterator,
        o: *const crate::q_list_of_q_web_engine_history_item::ConstIterator,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_iterator_operator__6(
        this_ptr: *const crate::q_list_of_q_web_engine_script::Iterator,
        o: *const crate::q_list_of_q_web_engine_script::ConstIterator,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_iterator_operator_4(
        this_ptr: *const crate::q_list_of_q_web_engine_history_item::Iterator,
        other: *const crate::q_list_of_q_web_engine_history_item::ConstIterator,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_iterator_operator_4(
        this_ptr: *const crate::q_list_of_q_web_engine_script::Iterator,
        other: *const crate::q_list_of_q_web_engine_script::ConstIterator,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_iterator_operator__8(
        this_ptr: *const crate::q_list_of_q_web_engine_history_item::Iterator,
        other: *const crate::q_list_of_q_web_engine_history_item::ConstIterator,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_iterator_operator__8(
        this_ptr: *const crate::q_list_of_q_web_engine_script::Iterator,
        other: *const crate::q_list_of_q_web_engine_script::ConstIterator,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_iterator_operator_5(
        this_ptr: *const crate::q_list_of_q_web_engine_history_item::Iterator,
        other: *const crate::q_list_of_q_web_engine_history_item::ConstIterator,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_iterator_operator_5(
        this_ptr: *const crate::q_list_of_q_web_engine_script::Iterator,
        other: *const crate::q_list_of_q_web_engine_script::ConstIterator,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_iterator_operator__9(
        this_ptr: *const crate::q_list_of_q_web_engine_history_item::Iterator,
        other: *const crate::q_list_of_q_web_engine_history_item::ConstIterator,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_iterator_operator__9(
        this_ptr: *const crate::q_list_of_q_web_engine_script::Iterator,
        other: *const crate::q_list_of_q_web_engine_script::ConstIterator,
    ) -> bool;

    /// <p>The prefix ++ operator (<code>++i</code>) advances the iterator to the next item in the hash and returns an iterator to the new current item.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qhash-iterator.html#operator-2b-2b">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>The prefix ++ operator (<code>++i</code>) advances the iterator to the next item in the hash and returns an iterator to the new current item.</p>
    /// <p>Calling this function on <a href="http://doc.qt.io/qt-5/qhash.html#end">QHash::end</a>() leads to undefined results.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qhash-iterator-obsolete.html#operator--">operator--</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_iterator_operator__10(
        this_ptr: *mut crate::q_list_of_q_web_engine_history_item::Iterator,
    ) -> *mut crate::q_list_of_q_web_engine_history_item::Iterator;

    /// <p>The prefix ++ operator (<code>++i</code>) advances the iterator to the next item in the hash and returns an iterator to the new current item.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qhash-iterator.html#operator-2b-2b">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>The prefix ++ operator (<code>++i</code>) advances the iterator to the next item in the hash and returns an iterator to the new current item.</p>
    /// <p>Calling this function on <a href="http://doc.qt.io/qt-5/qhash.html#end">QHash::end</a>() leads to undefined results.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qhash-iterator-obsolete.html#operator--">operator--</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_iterator_operator__10(
        this_ptr: *mut crate::q_list_of_q_web_engine_script::Iterator,
    ) -> *mut crate::q_list_of_q_web_engine_script::Iterator;

    /// <p>This is an overloaded function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qhash-iterator.html#operator-2b-2b-1">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This is an overloaded function.</p>
    /// <p>The postfix ++ operator (<code>i++</code>) advances the iterator to the next item in the hash and returns an iterator to the previously current item.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_iterator_operator__11(
        this_ptr: *mut crate::q_list_of_q_web_engine_history_item::Iterator,
        arg1: ::std::os::raw::c_int,
    ) -> *mut crate::q_list_of_q_web_engine_history_item::Iterator;

    /// <p>This is an overloaded function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qhash-iterator.html#operator-2b-2b-1">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This is an overloaded function.</p>
    /// <p>The postfix ++ operator (<code>i++</code>) advances the iterator to the next item in the hash and returns an iterator to the previously current item.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_iterator_operator__11(
        this_ptr: *mut crate::q_list_of_q_web_engine_script::Iterator,
        arg1: ::std::os::raw::c_int,
    ) -> *mut crate::q_list_of_q_web_engine_script::Iterator;

    /// <p>This function is obsolete. It is provided to keep old source code working. We strongly advise against using it in new code.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qhash-iterator-obsolete.html#operator--">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This function is obsolete. It is provided to keep old source code working. We strongly advise against using it in new code.</p>
    /// <p>This operator is deprecated in order to align with std::unordered_map functionality.</p>
    /// <p>The prefix -- operator (<code>--i</code>) makes the preceding item current and returns an iterator pointing to the new current item.</p>
    /// <p>Calling this function on <a href="http://doc.qt.io/qt-5/qhash.html#begin">QHash::begin</a>() leads to undefined results.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qhash-iterator.html#operator-2b-2b">operator++</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_iterator_operator__12(
        this_ptr: *mut crate::q_list_of_q_web_engine_history_item::Iterator,
    ) -> *mut crate::q_list_of_q_web_engine_history_item::Iterator;

    /// <p>This function is obsolete. It is provided to keep old source code working. We strongly advise against using it in new code.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qhash-iterator-obsolete.html#operator--">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This function is obsolete. It is provided to keep old source code working. We strongly advise against using it in new code.</p>
    /// <p>This operator is deprecated in order to align with std::unordered_map functionality.</p>
    /// <p>The prefix -- operator (<code>--i</code>) makes the preceding item current and returns an iterator pointing to the new current item.</p>
    /// <p>Calling this function on <a href="http://doc.qt.io/qt-5/qhash.html#begin">QHash::begin</a>() leads to undefined results.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qhash-iterator.html#operator-2b-2b">operator++</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_iterator_operator__12(
        this_ptr: *mut crate::q_list_of_q_web_engine_script::Iterator,
    ) -> *mut crate::q_list_of_q_web_engine_script::Iterator;

    /// <p>This function is obsolete. It is provided to keep old source code working. We strongly advise against using it in new code.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qhash-iterator-obsolete.html#operator---1">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This function is obsolete. It is provided to keep old source code working. We strongly advise against using it in new code.</p>
    /// <p>This operator is deprecated in order to align with std::unordered_map functionality.</p>
    /// <p>This is an overloaded function.</p>
    /// <p>The postfix -- operator (<code>i--</code>) makes the preceding item current and returns an iterator pointing to the previously current item.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_iterator_operator__13(
        this_ptr: *mut crate::q_list_of_q_web_engine_history_item::Iterator,
        arg1: ::std::os::raw::c_int,
    ) -> *mut crate::q_list_of_q_web_engine_history_item::Iterator;

    /// <p>This function is obsolete. It is provided to keep old source code working. We strongly advise against using it in new code.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qhash-iterator-obsolete.html#operator---1">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This function is obsolete. It is provided to keep old source code working. We strongly advise against using it in new code.</p>
    /// <p>This operator is deprecated in order to align with std::unordered_map functionality.</p>
    /// <p>This is an overloaded function.</p>
    /// <p>The postfix -- operator (<code>i--</code>) makes the preceding item current and returns an iterator pointing to the previously current item.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_iterator_operator__13(
        this_ptr: *mut crate::q_list_of_q_web_engine_script::Iterator,
        arg1: ::std::os::raw::c_int,
    ) -> *mut crate::q_list_of_q_web_engine_script::Iterator;

    /// <p>This function is obsolete. It is provided to keep old source code working. We strongly advise against using it in new code.</p>
    ///
    /// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qhash-iterator-obsolete.html#operator-2b-eq">C++ documentation</a> for <span style='color: green;'>```iterator &iterator::operator+=(int j)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This function is obsolete. It is provided to keep old source code working. We strongly advise against using it in new code.</p>
    /// <p>This operator is deprecated in order to align with std::unordered_map functionality.</p>
    /// <p>Advances the iterator by <i>j</i> items. (If <i>j</i> is negative, the iterator goes backward.)</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qhash-iterator-obsolete.html#operator--eq">operator-=</a>() and <a href="http://doc.qt.io/qt-5/qhash-iterator-obsolete.html#operator-2b">operator+</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_iterator_operator__14(
        this_ptr: *mut crate::q_list_of_q_web_engine_history_item::Iterator,
        j: ::std::os::raw::c_longlong,
    ) -> *mut crate::q_list_of_q_web_engine_history_item::Iterator;

    /// <p>This function is obsolete. It is provided to keep old source code working. We strongly advise against using it in new code.</p>
    ///
    /// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qhash-iterator-obsolete.html#operator-2b-eq">C++ documentation</a> for <span style='color: green;'>```iterator &iterator::operator+=(int j)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This function is obsolete. It is provided to keep old source code working. We strongly advise against using it in new code.</p>
    /// <p>This operator is deprecated in order to align with std::unordered_map functionality.</p>
    /// <p>Advances the iterator by <i>j</i> items. (If <i>j</i> is negative, the iterator goes backward.)</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qhash-iterator-obsolete.html#operator--eq">operator-=</a>() and <a href="http://doc.qt.io/qt-5/qhash-iterator-obsolete.html#operator-2b">operator+</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_iterator_operator__14(
        this_ptr: *mut crate::q_list_of_q_web_engine_script::Iterator,
        j: ::std::os::raw::c_longlong,
    ) -> *mut crate::q_list_of_q_web_engine_script::Iterator;

    /// <p>This function is obsolete. It is provided to keep old source code working. We strongly advise against using it in new code.</p>
    ///
    /// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qhash-iterator-obsolete.html#operator--eq">C++ documentation</a> for <span style='color: green;'>```iterator &iterator::operator-=(int j)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This function is obsolete. It is provided to keep old source code working. We strongly advise against using it in new code.</p>
    /// <p>This operator is deprecated in order to align with std::unordered_map functionality.</p>
    /// <p>Makes the iterator go back by <i>j</i> items. (If <i>j</i> is negative, the iterator goes forward.)</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qhash-iterator-obsolete.html#operator-2b-eq">operator+=</a>() and <a href="http://doc.qt.io/qt-5/qhash-iterator-obsolete.html#operator-">operator-</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_iterator_operator__15(
        this_ptr: *mut crate::q_list_of_q_web_engine_history_item::Iterator,
        j: ::std::os::raw::c_longlong,
    ) -> *mut crate::q_list_of_q_web_engine_history_item::Iterator;

    /// <p>This function is obsolete. It is provided to keep old source code working. We strongly advise against using it in new code.</p>
    ///
    /// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qhash-iterator-obsolete.html#operator--eq">C++ documentation</a> for <span style='color: green;'>```iterator &iterator::operator-=(int j)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This function is obsolete. It is provided to keep old source code working. We strongly advise against using it in new code.</p>
    /// <p>This operator is deprecated in order to align with std::unordered_map functionality.</p>
    /// <p>Makes the iterator go back by <i>j</i> items. (If <i>j</i> is negative, the iterator goes forward.)</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qhash-iterator-obsolete.html#operator-2b-eq">operator+=</a>() and <a href="http://doc.qt.io/qt-5/qhash-iterator-obsolete.html#operator-">operator-</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_iterator_operator__15(
        this_ptr: *mut crate::q_list_of_q_web_engine_script::Iterator,
        j: ::std::os::raw::c_longlong,
    ) -> *mut crate::q_list_of_q_web_engine_script::Iterator;

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_iterator_operator_6(
        this_ptr: *const crate::q_list_of_q_web_engine_history_item::Iterator,
        j: ::std::os::raw::c_longlong,
    ) -> *mut crate::q_list_of_q_web_engine_history_item::Iterator;

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_iterator_operator_6(
        this_ptr: *const crate::q_list_of_q_web_engine_script::Iterator,
        j: ::std::os::raw::c_longlong,
    ) -> *mut crate::q_list_of_q_web_engine_script::Iterator;

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_iterator_operator_7(
        this_ptr: *const crate::q_list_of_q_web_engine_history_item::Iterator,
        j: ::std::os::raw::c_longlong,
    ) -> *mut crate::q_list_of_q_web_engine_history_item::Iterator;

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_iterator_operator_7(
        this_ptr: *const crate::q_list_of_q_web_engine_script::Iterator,
        j: ::std::os::raw::c_longlong,
    ) -> *mut crate::q_list_of_q_web_engine_script::Iterator;

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_iterator_operator_8(
        this_ptr: *const crate::q_list_of_q_web_engine_history_item::Iterator,
        j: *const crate::q_list_of_q_web_engine_history_item::Iterator,
    ) -> ::std::os::raw::c_int;

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_iterator_operator_8(
        this_ptr: *const crate::q_list_of_q_web_engine_script::Iterator,
        j: *const crate::q_list_of_q_web_engine_script::Iterator,
    ) -> ::std::os::raw::c_int;

    /// <p>Constructs an uninitialized iterator.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist-const-iterator.html#const_iterator">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Constructs an uninitialized iterator.</p>
    /// <p>Functions like operator*() and operator++() should not be called on an uninitialized iterator. Use operator=() to assign a value to it before using it.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#constBegin">QList::constBegin</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#constEnd">QList::constEnd</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_const_iterator_const_iterator(
    ) -> *mut crate::q_list_of_q_web_engine_history_item::ConstIterator;

    /// <p>Constructs an uninitialized iterator.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist-const-iterator.html#const_iterator">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Constructs an uninitialized iterator.</p>
    /// <p>Functions like operator*() and operator++() should not be called on an uninitialized iterator. Use operator=() to assign a value to it before using it.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#constBegin">QList::constBegin</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#constEnd">QList::constEnd</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_const_iterator_const_iterator(
    ) -> *mut crate::q_list_of_q_web_engine_script::ConstIterator;

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_const_iterator_const_iterator1(
        o: *const crate::q_list_of_q_web_engine_history_item::ConstIterator,
    ) -> *mut crate::q_list_of_q_web_engine_history_item::ConstIterator;

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_const_iterator_const_iterator1(
        o: *const crate::q_list_of_q_web_engine_script::ConstIterator,
    ) -> *mut crate::q_list_of_q_web_engine_script::ConstIterator;

    /// <p>Assigns <i>other</i> to this.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qassociativeiterable-const-iterator.html#operator-eq">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Assigns <i>other</i> to this.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_const_iterator_operator_(
        this_ptr: *mut crate::q_list_of_q_web_engine_history_item::ConstIterator,
        o: *const crate::q_list_of_q_web_engine_history_item::ConstIterator,
    ) -> *mut crate::q_list_of_q_web_engine_history_item::ConstIterator;

    /// <p>Assigns <i>other</i> to this.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qassociativeiterable-const-iterator.html#operator-eq">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Assigns <i>other</i> to this.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_const_iterator_operator_(
        this_ptr: *mut crate::q_list_of_q_web_engine_script::ConstIterator,
        o: *const crate::q_list_of_q_web_engine_script::ConstIterator,
    ) -> *mut crate::q_list_of_q_web_engine_script::ConstIterator;

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_const_iterator_const_iterator2(
        o: *const crate::q_list_of_q_web_engine_history_item::Iterator,
    ) -> *mut crate::q_list_of_q_web_engine_history_item::ConstIterator;

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_const_iterator_const_iterator2(
        o: *const crate::q_list_of_q_web_engine_script::Iterator,
    ) -> *mut crate::q_list_of_q_web_engine_script::ConstIterator;

    /// <p>Returns the current value, converted to a <a href="http://doc.qt.io/qt-5/qvariant.html">QVariant</a>.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qassociativeiterable-const-iterator.html#operator-2a">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns the current value, converted to a <a href="http://doc.qt.io/qt-5/qvariant.html">QVariant</a>.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_const_iterator_operator_1(
        this_ptr: *const crate::q_list_of_q_web_engine_history_item::ConstIterator,
    ) -> *const crate::QWebEngineHistoryItem;

    /// <p>Returns the current value, converted to a <a href="http://doc.qt.io/qt-5/qvariant.html">QVariant</a>.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qassociativeiterable-const-iterator.html#operator-2a">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns the current value, converted to a <a href="http://doc.qt.io/qt-5/qvariant.html">QVariant</a>.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_const_iterator_operator_1(
        this_ptr: *const crate::q_list_of_q_web_engine_script::ConstIterator,
    ) -> *const crate::QWebEngineScript;

    /// <p>Returns a pointer to the current result.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qfuture-const-iterator.html#operator--gt">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns a pointer to the current result.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_const_iterator_operator__(
        this_ptr: *const crate::q_list_of_q_web_engine_history_item::ConstIterator,
    ) -> *const crate::QWebEngineHistoryItem;

    /// <p>Returns a pointer to the current result.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qfuture-const-iterator.html#operator--gt">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns a pointer to the current result.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_const_iterator_operator__(
        this_ptr: *const crate::q_list_of_q_web_engine_script::ConstIterator,
    ) -> *const crate::QWebEngineScript;

    /// <p>Returns the item at offset <i>j</i> from the item pointed to by this iterator (the item at position <code>*this + j</code>).</p>
    ///
    /// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qjsonarray-const-iterator.html#operator-5b-5d">C++ documentation</a> for <span style='color: green;'>```QJsonValue const_iterator::operator[](int j) const```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns the item at offset <i>j</i> from the item pointed to by this iterator (the item at position <code>*this + j</code>).</p>
    /// <p>This function is provided to make <a href="http://doc.qt.io/qt-5/qjsonarray.html">QJsonArray</a> iterators behave like C++ pointers.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qjsonarray-const-iterator.html#operator-2b">operator+</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_const_iterator_operator__1(
        this_ptr: *const crate::q_list_of_q_web_engine_history_item::ConstIterator,
        j: ::std::os::raw::c_longlong,
    ) -> *const crate::QWebEngineHistoryItem;

    /// <p>Returns the item at offset <i>j</i> from the item pointed to by this iterator (the item at position <code>*this + j</code>).</p>
    ///
    /// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qjsonarray-const-iterator.html#operator-5b-5d">C++ documentation</a> for <span style='color: green;'>```QJsonValue const_iterator::operator[](int j) const```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns the item at offset <i>j</i> from the item pointed to by this iterator (the item at position <code>*this + j</code>).</p>
    /// <p>This function is provided to make <a href="http://doc.qt.io/qt-5/qjsonarray.html">QJsonArray</a> iterators behave like C++ pointers.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qjsonarray-const-iterator.html#operator-2b">operator+</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_const_iterator_operator__1(
        this_ptr: *const crate::q_list_of_q_web_engine_script::ConstIterator,
        j: ::std::os::raw::c_longlong,
    ) -> *const crate::QWebEngineScript;

    /// <p>Returns <code>true</code> if <i>other</i> points to the same item as this iterator; otherwise returns <code>false</code>.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qassociativeiterable-const-iterator.html#operator-eq-eq">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if <i>other</i> points to the same item as this iterator; otherwise returns <code>false</code>.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qassociativeiterable-const-iterator.html#operator-not-eq">operator!=</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_const_iterator_operator__2(
        this_ptr: *const crate::q_list_of_q_web_engine_history_item::ConstIterator,
        o: *const crate::q_list_of_q_web_engine_history_item::ConstIterator,
    ) -> bool;

    /// <p>Returns <code>true</code> if <i>other</i> points to the same item as this iterator; otherwise returns <code>false</code>.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qassociativeiterable-const-iterator.html#operator-eq-eq">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if <i>other</i> points to the same item as this iterator; otherwise returns <code>false</code>.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qassociativeiterable-const-iterator.html#operator-not-eq">operator!=</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_const_iterator_operator__2(
        this_ptr: *const crate::q_list_of_q_web_engine_script::ConstIterator,
        o: *const crate::q_list_of_q_web_engine_script::ConstIterator,
    ) -> bool;

    /// <p>Returns <code>true</code> if the item pointed to by this iterator is less than the item pointed to by the <i>other</i> iterator.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qjsonarray-const-iterator.html#operator-lt">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if the item pointed to by this iterator is less than the item pointed to by the <i>other</i> iterator.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_const_iterator_operator_2(
        this_ptr: *const crate::q_list_of_q_web_engine_history_item::ConstIterator,
        other: *const crate::q_list_of_q_web_engine_history_item::ConstIterator,
    ) -> bool;

    /// <p>Returns <code>true</code> if the item pointed to by this iterator is less than the item pointed to by the <i>other</i> iterator.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qjsonarray-const-iterator.html#operator-lt">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if the item pointed to by this iterator is less than the item pointed to by the <i>other</i> iterator.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_const_iterator_operator_2(
        this_ptr: *const crate::q_list_of_q_web_engine_script::ConstIterator,
        other: *const crate::q_list_of_q_web_engine_script::ConstIterator,
    ) -> bool;

    /// <p>Returns <code>true</code> if the item pointed to by this iterator is less than or equal to the item pointed to by the <i>other</i> iterator.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qjsonarray-const-iterator.html#operator-lt-eq">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if the item pointed to by this iterator is less than or equal to the item pointed to by the <i>other</i> iterator.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_const_iterator_operator__4(
        this_ptr: *const crate::q_list_of_q_web_engine_history_item::ConstIterator,
        other: *const crate::q_list_of_q_web_engine_history_item::ConstIterator,
    ) -> bool;

    /// <p>Returns <code>true</code> if the item pointed to by this iterator is less than or equal to the item pointed to by the <i>other</i> iterator.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qjsonarray-const-iterator.html#operator-lt-eq">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if the item pointed to by this iterator is less than or equal to the item pointed to by the <i>other</i> iterator.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_const_iterator_operator__4(
        this_ptr: *const crate::q_list_of_q_web_engine_script::ConstIterator,
        other: *const crate::q_list_of_q_web_engine_script::ConstIterator,
    ) -> bool;

    /// <p>Returns <code>true</code> if the item pointed to by this iterator is greater than the item pointed to by the <i>other</i> iterator.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qjsonarray-const-iterator.html#operator-gt">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if the item pointed to by this iterator is greater than the item pointed to by the <i>other</i> iterator.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_const_iterator_operator_3(
        this_ptr: *const crate::q_list_of_q_web_engine_history_item::ConstIterator,
        other: *const crate::q_list_of_q_web_engine_history_item::ConstIterator,
    ) -> bool;

    /// <p>Returns <code>true</code> if the item pointed to by this iterator is greater than the item pointed to by the <i>other</i> iterator.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qjsonarray-const-iterator.html#operator-gt">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if the item pointed to by this iterator is greater than the item pointed to by the <i>other</i> iterator.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_const_iterator_operator_3(
        this_ptr: *const crate::q_list_of_q_web_engine_script::ConstIterator,
        other: *const crate::q_list_of_q_web_engine_script::ConstIterator,
    ) -> bool;

    /// <p>Returns <code>true</code> if the item pointed to by this iterator is greater than or equal to the item pointed to by the <i>other</i> iterator.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qjsonarray-const-iterator.html#operator-gt-eq">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if the item pointed to by this iterator is greater than or equal to the item pointed to by the <i>other</i> iterator.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_const_iterator_operator__5(
        this_ptr: *const crate::q_list_of_q_web_engine_history_item::ConstIterator,
        other: *const crate::q_list_of_q_web_engine_history_item::ConstIterator,
    ) -> bool;

    /// <p>Returns <code>true</code> if the item pointed to by this iterator is greater than or equal to the item pointed to by the <i>other</i> iterator.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qjsonarray-const-iterator.html#operator-gt-eq">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if the item pointed to by this iterator is greater than or equal to the item pointed to by the <i>other</i> iterator.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_const_iterator_operator__5(
        this_ptr: *const crate::q_list_of_q_web_engine_script::ConstIterator,
        other: *const crate::q_list_of_q_web_engine_script::ConstIterator,
    ) -> bool;

    /// <p>The prefix ++ operator (<code>++it</code>) advances the iterator to the next item in the container and returns an iterator to the new current item.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qassociativeiterable-const-iterator.html#operator-2b-2b">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>The prefix ++ operator (<code>++it</code>) advances the iterator to the next item in the container and returns an iterator to the new current item.</p>
    /// <p>Calling this function on <a href="http://doc.qt.io/qt-5/qassociativeiterable.html#end">QAssociativeIterable::end</a>() leads to undefined results.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qassociativeiterable-const-iterator.html#operator--">operator--</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_const_iterator_operator__6(
        this_ptr: *mut crate::q_list_of_q_web_engine_history_item::ConstIterator,
    ) -> *mut crate::q_list_of_q_web_engine_history_item::ConstIterator;

    /// <p>The prefix ++ operator (<code>++it</code>) advances the iterator to the next item in the container and returns an iterator to the new current item.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qassociativeiterable-const-iterator.html#operator-2b-2b">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>The prefix ++ operator (<code>++it</code>) advances the iterator to the next item in the container and returns an iterator to the new current item.</p>
    /// <p>Calling this function on <a href="http://doc.qt.io/qt-5/qassociativeiterable.html#end">QAssociativeIterable::end</a>() leads to undefined results.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qassociativeiterable-const-iterator.html#operator--">operator--</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_const_iterator_operator__6(
        this_ptr: *mut crate::q_list_of_q_web_engine_script::ConstIterator,
    ) -> *mut crate::q_list_of_q_web_engine_script::ConstIterator;

    /// <p>This is an overloaded function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qassociativeiterable-const-iterator.html#operator-2b-2b-1">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This is an overloaded function.</p>
    /// <p>The postfix ++ operator (<code>it++</code>) advances the iterator to the next item in the container and returns an iterator to the previously current item.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_const_iterator_operator__7(
        this_ptr: *mut crate::q_list_of_q_web_engine_history_item::ConstIterator,
        arg1: ::std::os::raw::c_int,
    ) -> *mut crate::q_list_of_q_web_engine_history_item::ConstIterator;

    /// <p>This is an overloaded function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qassociativeiterable-const-iterator.html#operator-2b-2b-1">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This is an overloaded function.</p>
    /// <p>The postfix ++ operator (<code>it++</code>) advances the iterator to the next item in the container and returns an iterator to the previously current item.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_const_iterator_operator__7(
        this_ptr: *mut crate::q_list_of_q_web_engine_script::ConstIterator,
        arg1: ::std::os::raw::c_int,
    ) -> *mut crate::q_list_of_q_web_engine_script::ConstIterator;

    /// <p>The prefix -- operator (<code>--it</code>) makes the preceding item current and returns an iterator to the new current item.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qassociativeiterable-const-iterator.html#operator--">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>The prefix -- operator (<code>--it</code>) makes the preceding item current and returns an iterator to the new current item.</p>
    /// <p>Calling this function on <a href="http://doc.qt.io/qt-5/qassociativeiterable.html#begin">QAssociativeIterable::begin</a>() leads to undefined results.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qassociativeiterable-const-iterator.html#operator-2b-2b">operator++</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_const_iterator_operator__8(
        this_ptr: *mut crate::q_list_of_q_web_engine_history_item::ConstIterator,
    ) -> *mut crate::q_list_of_q_web_engine_history_item::ConstIterator;

    /// <p>The prefix -- operator (<code>--it</code>) makes the preceding item current and returns an iterator to the new current item.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qassociativeiterable-const-iterator.html#operator--">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>The prefix -- operator (<code>--it</code>) makes the preceding item current and returns an iterator to the new current item.</p>
    /// <p>Calling this function on <a href="http://doc.qt.io/qt-5/qassociativeiterable.html#begin">QAssociativeIterable::begin</a>() leads to undefined results.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qassociativeiterable-const-iterator.html#operator-2b-2b">operator++</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_const_iterator_operator__8(
        this_ptr: *mut crate::q_list_of_q_web_engine_script::ConstIterator,
    ) -> *mut crate::q_list_of_q_web_engine_script::ConstIterator;

    /// <p>This is an overloaded function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qassociativeiterable-const-iterator.html#operator---1">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This is an overloaded function.</p>
    /// <p>The postfix -- operator (<code>it--</code>) makes the preceding item current and returns an iterator to the previously current item.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_const_iterator_operator__9(
        this_ptr: *mut crate::q_list_of_q_web_engine_history_item::ConstIterator,
        arg1: ::std::os::raw::c_int,
    ) -> *mut crate::q_list_of_q_web_engine_history_item::ConstIterator;

    /// <p>This is an overloaded function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qassociativeiterable-const-iterator.html#operator---1">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This is an overloaded function.</p>
    /// <p>The postfix -- operator (<code>it--</code>) makes the preceding item current and returns an iterator to the previously current item.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_const_iterator_operator__9(
        this_ptr: *mut crate::q_list_of_q_web_engine_script::ConstIterator,
        arg1: ::std::os::raw::c_int,
    ) -> *mut crate::q_list_of_q_web_engine_script::ConstIterator;

    /// <p>Advances the iterator by <i>j</i> items.</p>
    ///
    /// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qassociativeiterable-const-iterator.html#operator-2b-eq">C++ documentation</a> for <span style='color: green;'>```const_iterator &const_iterator::operator+=(int j)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Advances the iterator by <i>j</i> items.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qassociativeiterable-const-iterator.html#operator--eq">operator-=</a>() and <a href="http://doc.qt.io/qt-5/qassociativeiterable-const-iterator.html#operator-2b">operator+</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_const_iterator_operator__10(
        this_ptr: *mut crate::q_list_of_q_web_engine_history_item::ConstIterator,
        j: ::std::os::raw::c_longlong,
    ) -> *mut crate::q_list_of_q_web_engine_history_item::ConstIterator;

    /// <p>Advances the iterator by <i>j</i> items.</p>
    ///
    /// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qassociativeiterable-const-iterator.html#operator-2b-eq">C++ documentation</a> for <span style='color: green;'>```const_iterator &const_iterator::operator+=(int j)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Advances the iterator by <i>j</i> items.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qassociativeiterable-const-iterator.html#operator--eq">operator-=</a>() and <a href="http://doc.qt.io/qt-5/qassociativeiterable-const-iterator.html#operator-2b">operator+</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_const_iterator_operator__10(
        this_ptr: *mut crate::q_list_of_q_web_engine_script::ConstIterator,
        j: ::std::os::raw::c_longlong,
    ) -> *mut crate::q_list_of_q_web_engine_script::ConstIterator;

    /// <p>Makes the iterator go back by <i>j</i> items.</p>
    ///
    /// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qassociativeiterable-const-iterator.html#operator--eq">C++ documentation</a> for <span style='color: green;'>```const_iterator &const_iterator::operator-=(int j)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Makes the iterator go back by <i>j</i> items.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qassociativeiterable-const-iterator.html#operator-2b-eq">operator+=</a>() and <a href="http://doc.qt.io/qt-5/qassociativeiterable-const-iterator.html#operator-">operator-</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_const_iterator_operator__11(
        this_ptr: *mut crate::q_list_of_q_web_engine_history_item::ConstIterator,
        j: ::std::os::raw::c_longlong,
    ) -> *mut crate::q_list_of_q_web_engine_history_item::ConstIterator;

    /// <p>Makes the iterator go back by <i>j</i> items.</p>
    ///
    /// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qassociativeiterable-const-iterator.html#operator--eq">C++ documentation</a> for <span style='color: green;'>```const_iterator &const_iterator::operator-=(int j)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Makes the iterator go back by <i>j</i> items.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qassociativeiterable-const-iterator.html#operator-2b-eq">operator+=</a>() and <a href="http://doc.qt.io/qt-5/qassociativeiterable-const-iterator.html#operator-">operator-</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_const_iterator_operator__11(
        this_ptr: *mut crate::q_list_of_q_web_engine_script::ConstIterator,
        j: ::std::os::raw::c_longlong,
    ) -> *mut crate::q_list_of_q_web_engine_script::ConstIterator;

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_const_iterator_operator_4(
        this_ptr: *const crate::q_list_of_q_web_engine_history_item::ConstIterator,
        j: ::std::os::raw::c_longlong,
    ) -> *mut crate::q_list_of_q_web_engine_history_item::ConstIterator;

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_const_iterator_operator_4(
        this_ptr: *const crate::q_list_of_q_web_engine_script::ConstIterator,
        j: ::std::os::raw::c_longlong,
    ) -> *mut crate::q_list_of_q_web_engine_script::ConstIterator;

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_const_iterator_operator_5(
        this_ptr: *const crate::q_list_of_q_web_engine_history_item::ConstIterator,
        j: ::std::os::raw::c_longlong,
    ) -> *mut crate::q_list_of_q_web_engine_history_item::ConstIterator;

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_const_iterator_operator_5(
        this_ptr: *const crate::q_list_of_q_web_engine_script::ConstIterator,
        j: ::std::os::raw::c_longlong,
    ) -> *mut crate::q_list_of_q_web_engine_script::ConstIterator;

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_const_iterator_operator_6(
        this_ptr: *const crate::q_list_of_q_web_engine_history_item::ConstIterator,
        j: *const crate::q_list_of_q_web_engine_history_item::ConstIterator,
    ) -> ::std::os::raw::c_int;

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_const_iterator_operator_6(
        this_ptr: *const crate::q_list_of_q_web_engine_script::ConstIterator,
        j: *const crate::q_list_of_q_web_engine_script::ConstIterator,
    ) -> ::std::os::raw::c_int;

    /// <p>Returns an <a href="http://doc.qt.io/qt-5/containers.html#stl-style-iterators">STL-style iterator</a> pointing to the first item in the list.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#begin">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns an <a href="http://doc.qt.io/qt-5/containers.html#stl-style-iterators">STL-style iterator</a> pointing to the first item in the list.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#constBegin">constBegin</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#end">end</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_begin(
        this_ptr: *mut crate::QListOfQWebEngineHistoryItem,
    ) -> *mut crate::q_list_of_q_web_engine_history_item::Iterator;

    /// <p>Returns an <a href="http://doc.qt.io/qt-5/containers.html#stl-style-iterators">STL-style iterator</a> pointing to the first item in the list.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#begin">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns an <a href="http://doc.qt.io/qt-5/containers.html#stl-style-iterators">STL-style iterator</a> pointing to the first item in the list.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#constBegin">constBegin</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#end">end</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_begin(
        this_ptr: *mut crate::QListOfQWebEngineScript,
    ) -> *mut crate::q_list_of_q_web_engine_script::Iterator;

    /// <p>Returns an <a href="http://doc.qt.io/qt-5/containers.html#stl-style-iterators">STL-style iterator</a> pointing to the first item in the list.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#begin">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns an <a href="http://doc.qt.io/qt-5/containers.html#stl-style-iterators">STL-style iterator</a> pointing to the first item in the list.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#constBegin">constBegin</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#end">end</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_begin1(
        this_ptr: *const crate::QListOfQWebEngineHistoryItem,
    ) -> *mut crate::q_list_of_q_web_engine_history_item::ConstIterator;

    /// <p>Returns an <a href="http://doc.qt.io/qt-5/containers.html#stl-style-iterators">STL-style iterator</a> pointing to the first item in the list.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#begin">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns an <a href="http://doc.qt.io/qt-5/containers.html#stl-style-iterators">STL-style iterator</a> pointing to the first item in the list.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#constBegin">constBegin</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#end">end</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_begin1(
        this_ptr: *const crate::QListOfQWebEngineScript,
    ) -> *mut crate::q_list_of_q_web_engine_script::ConstIterator;

    /// <p>Returns a const <a href="http://doc.qt.io/qt-5/containers.html#stl-style-iterators">STL-style iterator</a> pointing to the first item in the list.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#cbegin">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns a const <a href="http://doc.qt.io/qt-5/containers.html#stl-style-iterators">STL-style iterator</a> pointing to the first item in the list.</p>
    /// <p>This function was introduced in Qt 5.0.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#begin">begin</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#cend">cend</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_cbegin(
        this_ptr: *const crate::QListOfQWebEngineHistoryItem,
    ) -> *mut crate::q_list_of_q_web_engine_history_item::ConstIterator;

    /// <p>Returns a const <a href="http://doc.qt.io/qt-5/containers.html#stl-style-iterators">STL-style iterator</a> pointing to the first item in the list.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#cbegin">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns a const <a href="http://doc.qt.io/qt-5/containers.html#stl-style-iterators">STL-style iterator</a> pointing to the first item in the list.</p>
    /// <p>This function was introduced in Qt 5.0.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#begin">begin</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#cend">cend</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_cbegin(
        this_ptr: *const crate::QListOfQWebEngineScript,
    ) -> *mut crate::q_list_of_q_web_engine_script::ConstIterator;

    /// <p>Returns a const <a href="http://doc.qt.io/qt-5/containers.html#stl-style-iterators">STL-style iterator</a> pointing to the first item in the list.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#constBegin">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns a const <a href="http://doc.qt.io/qt-5/containers.html#stl-style-iterators">STL-style iterator</a> pointing to the first item in the list.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#begin">begin</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#constEnd">constEnd</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_constBegin(
        this_ptr: *const crate::QListOfQWebEngineHistoryItem,
    ) -> *mut crate::q_list_of_q_web_engine_history_item::ConstIterator;

    /// <p>Returns a const <a href="http://doc.qt.io/qt-5/containers.html#stl-style-iterators">STL-style iterator</a> pointing to the first item in the list.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#constBegin">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns a const <a href="http://doc.qt.io/qt-5/containers.html#stl-style-iterators">STL-style iterator</a> pointing to the first item in the list.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#begin">begin</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#constEnd">constEnd</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_constBegin(
        this_ptr: *const crate::QListOfQWebEngineScript,
    ) -> *mut crate::q_list_of_q_web_engine_script::ConstIterator;

    /// <p>Returns an <a href="http://doc.qt.io/qt-5/containers.html#stl-style-iterators">STL-style iterator</a> pointing to the imaginary item after the last item in the list.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#end">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns an <a href="http://doc.qt.io/qt-5/containers.html#stl-style-iterators">STL-style iterator</a> pointing to the imaginary item after the last item in the list.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#begin">begin</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#constEnd">constEnd</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_end(
        this_ptr: *mut crate::QListOfQWebEngineHistoryItem,
    ) -> *mut crate::q_list_of_q_web_engine_history_item::Iterator;

    /// <p>Returns an <a href="http://doc.qt.io/qt-5/containers.html#stl-style-iterators">STL-style iterator</a> pointing to the imaginary item after the last item in the list.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#end">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns an <a href="http://doc.qt.io/qt-5/containers.html#stl-style-iterators">STL-style iterator</a> pointing to the imaginary item after the last item in the list.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#begin">begin</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#constEnd">constEnd</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_end(
        this_ptr: *mut crate::QListOfQWebEngineScript,
    ) -> *mut crate::q_list_of_q_web_engine_script::Iterator;

    /// <p>Returns an <a href="http://doc.qt.io/qt-5/containers.html#stl-style-iterators">STL-style iterator</a> pointing to the imaginary item after the last item in the list.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#end">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns an <a href="http://doc.qt.io/qt-5/containers.html#stl-style-iterators">STL-style iterator</a> pointing to the imaginary item after the last item in the list.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#begin">begin</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#constEnd">constEnd</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_end1(
        this_ptr: *const crate::QListOfQWebEngineHistoryItem,
    ) -> *mut crate::q_list_of_q_web_engine_history_item::ConstIterator;

    /// <p>Returns an <a href="http://doc.qt.io/qt-5/containers.html#stl-style-iterators">STL-style iterator</a> pointing to the imaginary item after the last item in the list.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#end">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns an <a href="http://doc.qt.io/qt-5/containers.html#stl-style-iterators">STL-style iterator</a> pointing to the imaginary item after the last item in the list.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#begin">begin</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#constEnd">constEnd</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_end1(
        this_ptr: *const crate::QListOfQWebEngineScript,
    ) -> *mut crate::q_list_of_q_web_engine_script::ConstIterator;

    /// <p>Returns a const <a href="http://doc.qt.io/qt-5/containers.html#stl-style-iterators">STL-style iterator</a> pointing to the imaginary item after the last item in the list.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#cend">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns a const <a href="http://doc.qt.io/qt-5/containers.html#stl-style-iterators">STL-style iterator</a> pointing to the imaginary item after the last item in the list.</p>
    /// <p>This function was introduced in Qt 5.0.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#cbegin">cbegin</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#end">end</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_cend(
        this_ptr: *const crate::QListOfQWebEngineHistoryItem,
    ) -> *mut crate::q_list_of_q_web_engine_history_item::ConstIterator;

    /// <p>Returns a const <a href="http://doc.qt.io/qt-5/containers.html#stl-style-iterators">STL-style iterator</a> pointing to the imaginary item after the last item in the list.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#cend">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns a const <a href="http://doc.qt.io/qt-5/containers.html#stl-style-iterators">STL-style iterator</a> pointing to the imaginary item after the last item in the list.</p>
    /// <p>This function was introduced in Qt 5.0.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#cbegin">cbegin</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#end">end</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_cend(
        this_ptr: *const crate::QListOfQWebEngineScript,
    ) -> *mut crate::q_list_of_q_web_engine_script::ConstIterator;

    /// <p>Returns a const <a href="http://doc.qt.io/qt-5/containers.html#stl-style-iterators">STL-style iterator</a> pointing to the imaginary item after the last item in the list.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#constEnd">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns a const <a href="http://doc.qt.io/qt-5/containers.html#stl-style-iterators">STL-style iterator</a> pointing to the imaginary item after the last item in the list.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#constBegin">constBegin</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#end">end</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_constEnd(
        this_ptr: *const crate::QListOfQWebEngineHistoryItem,
    ) -> *mut crate::q_list_of_q_web_engine_history_item::ConstIterator;

    /// <p>Returns a const <a href="http://doc.qt.io/qt-5/containers.html#stl-style-iterators">STL-style iterator</a> pointing to the imaginary item after the last item in the list.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#constEnd">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns a const <a href="http://doc.qt.io/qt-5/containers.html#stl-style-iterators">STL-style iterator</a> pointing to the imaginary item after the last item in the list.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#constBegin">constBegin</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#end">end</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_constEnd(
        this_ptr: *const crate::QListOfQWebEngineScript,
    ) -> *mut crate::q_list_of_q_web_engine_script::ConstIterator;

    /// <p>This is an overloaded function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#insert-1">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This is an overloaded function.</p>
    /// <p>Inserts <i>value</i> in front of the item pointed to by the iterator <i>before</i>. Returns an iterator pointing at the inserted item. Note that the iterator passed to the function will be invalid after the call; the returned iterator should be used instead.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_insert1(
        this_ptr: *mut crate::QListOfQWebEngineHistoryItem,
        before: *const crate::q_list_of_q_web_engine_history_item::Iterator,
        t: *const crate::QWebEngineHistoryItem,
    ) -> *mut crate::q_list_of_q_web_engine_history_item::Iterator;

    /// <p>This is an overloaded function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#insert-1">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This is an overloaded function.</p>
    /// <p>Inserts <i>value</i> in front of the item pointed to by the iterator <i>before</i>. Returns an iterator pointing at the inserted item. Note that the iterator passed to the function will be invalid after the call; the returned iterator should be used instead.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_insert1(
        this_ptr: *mut crate::QListOfQWebEngineScript,
        before: *const crate::q_list_of_q_web_engine_script::Iterator,
        t: *const crate::QWebEngineScript,
    ) -> *mut crate::q_list_of_q_web_engine_script::Iterator;

    /// <p>Removes the item associated with the iterator <i>pos</i> from the list, and returns an iterator to the next item in the list (which may be <a href="http://doc.qt.io/qt-5/qlist.html#end">end</a>()).</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#erase">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Removes the item associated with the iterator <i>pos</i> from the list, and returns an iterator to the next item in the list (which may be <a href="http://doc.qt.io/qt-5/qlist.html#end">end</a>()).</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#insert">insert</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#removeAt">removeAt</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_erase(
        this_ptr: *mut crate::QListOfQWebEngineHistoryItem,
        pos: *const crate::q_list_of_q_web_engine_history_item::Iterator,
    ) -> *mut crate::q_list_of_q_web_engine_history_item::Iterator;

    /// <p>Removes the item associated with the iterator <i>pos</i> from the list, and returns an iterator to the next item in the list (which may be <a href="http://doc.qt.io/qt-5/qlist.html#end">end</a>()).</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#erase">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Removes the item associated with the iterator <i>pos</i> from the list, and returns an iterator to the next item in the list (which may be <a href="http://doc.qt.io/qt-5/qlist.html#end">end</a>()).</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#insert">insert</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#removeAt">removeAt</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_erase(
        this_ptr: *mut crate::QListOfQWebEngineScript,
        pos: *const crate::q_list_of_q_web_engine_script::Iterator,
    ) -> *mut crate::q_list_of_q_web_engine_script::Iterator;

    /// <p>This is an overloaded function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#erase-1">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This is an overloaded function.</p>
    /// <p>Removes all the items from <i>begin</i> up to (but not including) <i>end</i>. Returns an iterator to the same item that <i>end</i> referred to before the call.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_erase1(
        this_ptr: *mut crate::QListOfQWebEngineHistoryItem,
        first: *const crate::q_list_of_q_web_engine_history_item::Iterator,
        last: *const crate::q_list_of_q_web_engine_history_item::Iterator,
    ) -> *mut crate::q_list_of_q_web_engine_history_item::Iterator;

    /// <p>This is an overloaded function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#erase-1">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This is an overloaded function.</p>
    /// <p>Removes all the items from <i>begin</i> up to (but not including) <i>end</i>. Returns an iterator to the same item that <i>end</i> referred to before the call.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_erase1(
        this_ptr: *mut crate::QListOfQWebEngineScript,
        first: *const crate::q_list_of_q_web_engine_script::Iterator,
        last: *const crate::q_list_of_q_web_engine_script::Iterator,
    ) -> *mut crate::q_list_of_q_web_engine_script::Iterator;

    /// <p>Returns the number of items in the list. This is effectively the same as <a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>().</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#count-1">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns the number of items in the list. This is effectively the same as <a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_count1(
        this_ptr: *const crate::QListOfQWebEngineHistoryItem,
    ) -> ::std::os::raw::c_int;

    /// <p>Returns the number of items in the list. This is effectively the same as <a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>().</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#count-1">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns the number of items in the list. This is effectively the same as <a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_count1(
        this_ptr: *const crate::QListOfQWebEngineScript,
    ) -> ::std::os::raw::c_int;

    /// <p>This function is identical to <a href="http://doc.qt.io/qt-5/qlist.html#count-1">count</a>().</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#length">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This function is identical to <a href="http://doc.qt.io/qt-5/qlist.html#count-1">count</a>().</p>
    /// <p>This function was introduced in Qt 4.5.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#count-1">count</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_length(
        this_ptr: *const crate::QListOfQWebEngineHistoryItem,
    ) -> ::std::os::raw::c_int;

    /// <p>This function is identical to <a href="http://doc.qt.io/qt-5/qlist.html#count-1">count</a>().</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#length">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This function is identical to <a href="http://doc.qt.io/qt-5/qlist.html#count-1">count</a>().</p>
    /// <p>This function was introduced in Qt 4.5.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#count-1">count</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_length(
        this_ptr: *const crate::QListOfQWebEngineScript,
    ) -> ::std::os::raw::c_int;

    /// <p>Returns a reference to the first item in the list. The list must not be empty. If the list can be empty, call <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() before calling this function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#first">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns a reference to the first item in the list. The list must not be empty. If the list can be empty, call <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() before calling this function.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#constFirst">constFirst</a>(), <a href="http://doc.qt.io/qt-5/qlist.html#last">last</a>(), and <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_first(
        this_ptr: *mut crate::QListOfQWebEngineHistoryItem,
    ) -> *mut crate::QWebEngineHistoryItem;

    /// <p>Returns a reference to the first item in the list. The list must not be empty. If the list can be empty, call <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() before calling this function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#first">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns a reference to the first item in the list. The list must not be empty. If the list can be empty, call <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() before calling this function.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#constFirst">constFirst</a>(), <a href="http://doc.qt.io/qt-5/qlist.html#last">last</a>(), and <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_first(
        this_ptr: *mut crate::QListOfQWebEngineScript,
    ) -> *mut crate::QWebEngineScript;

    /// <p>Returns a const reference to the first item in the list. The list must not be empty. If the list can be empty, call <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() before calling this function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#constFirst">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns a const reference to the first item in the list. The list must not be empty. If the list can be empty, call <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() before calling this function.</p>
    /// <p>This function was introduced in Qt 5.6.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#constLast">constLast</a>(), <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>(), and <a href="http://doc.qt.io/qt-5/qlist.html#first">first</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_constFirst(
        this_ptr: *const crate::QListOfQWebEngineHistoryItem,
    ) -> *const crate::QWebEngineHistoryItem;

    /// <p>Returns a const reference to the first item in the list. The list must not be empty. If the list can be empty, call <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() before calling this function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#constFirst">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns a const reference to the first item in the list. The list must not be empty. If the list can be empty, call <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() before calling this function.</p>
    /// <p>This function was introduced in Qt 5.6.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#constLast">constLast</a>(), <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>(), and <a href="http://doc.qt.io/qt-5/qlist.html#first">first</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_constFirst(
        this_ptr: *const crate::QListOfQWebEngineScript,
    ) -> *const crate::QWebEngineScript;

    /// <p>This is an overloaded function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#first-1">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This is an overloaded function.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_first1(
        this_ptr: *const crate::QListOfQWebEngineHistoryItem,
    ) -> *const crate::QWebEngineHistoryItem;

    /// <p>This is an overloaded function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#first-1">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This is an overloaded function.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_first1(
        this_ptr: *const crate::QListOfQWebEngineScript,
    ) -> *const crate::QWebEngineScript;

    /// <p>Returns a reference to the last item in the list. The list must not be empty. If the list can be empty, call <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() before calling this function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#last">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns a reference to the last item in the list. The list must not be empty. If the list can be empty, call <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() before calling this function.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#constLast">constLast</a>(), <a href="http://doc.qt.io/qt-5/qlist.html#first">first</a>(), and <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_last(
        this_ptr: *mut crate::QListOfQWebEngineHistoryItem,
    ) -> *mut crate::QWebEngineHistoryItem;

    /// <p>Returns a reference to the last item in the list. The list must not be empty. If the list can be empty, call <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() before calling this function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#last">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns a reference to the last item in the list. The list must not be empty. If the list can be empty, call <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() before calling this function.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#constLast">constLast</a>(), <a href="http://doc.qt.io/qt-5/qlist.html#first">first</a>(), and <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_last(
        this_ptr: *mut crate::QListOfQWebEngineScript,
    ) -> *mut crate::QWebEngineScript;

    /// <p>This is an overloaded function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#last-1">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This is an overloaded function.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_last1(
        this_ptr: *const crate::QListOfQWebEngineHistoryItem,
    ) -> *const crate::QWebEngineHistoryItem;

    /// <p>This is an overloaded function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#last-1">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This is an overloaded function.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_last1(
        this_ptr: *const crate::QListOfQWebEngineScript,
    ) -> *const crate::QWebEngineScript;

    /// <p>Returns a reference to the last item in the list. The list must not be empty. If the list can be empty, call <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() before calling this function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#constLast">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns a reference to the last item in the list. The list must not be empty. If the list can be empty, call <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() before calling this function.</p>
    /// <p>This function was introduced in Qt 5.6.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#constFirst">constFirst</a>(), <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>(), and <a href="http://doc.qt.io/qt-5/qlist.html#last">last</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_constLast(
        this_ptr: *const crate::QListOfQWebEngineHistoryItem,
    ) -> *const crate::QWebEngineHistoryItem;

    /// <p>Returns a reference to the last item in the list. The list must not be empty. If the list can be empty, call <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() before calling this function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#constLast">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns a reference to the last item in the list. The list must not be empty. If the list can be empty, call <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() before calling this function.</p>
    /// <p>This function was introduced in Qt 5.6.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#constFirst">constFirst</a>(), <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>(), and <a href="http://doc.qt.io/qt-5/qlist.html#last">last</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_constLast(
        this_ptr: *const crate::QListOfQWebEngineScript,
    ) -> *const crate::QWebEngineScript;

    /// <p>Removes the first item in the list. Calling this function is equivalent to calling <a href="http://doc.qt.io/qt-5/qlist.html#removeAt">removeAt</a>(0). The list must not be empty. If the list can be empty, call <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() before calling this function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#removeFirst">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Removes the first item in the list. Calling this function is equivalent to calling <a href="http://doc.qt.io/qt-5/qlist.html#removeAt">removeAt</a>(0). The list must not be empty. If the list can be empty, call <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() before calling this function.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#removeAt">removeAt</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#takeFirst">takeFirst</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_removeFirst(
        this_ptr: *mut crate::QListOfQWebEngineHistoryItem,
    );

    /// <p>Removes the first item in the list. Calling this function is equivalent to calling <a href="http://doc.qt.io/qt-5/qlist.html#removeAt">removeAt</a>(0). The list must not be empty. If the list can be empty, call <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() before calling this function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#removeFirst">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Removes the first item in the list. Calling this function is equivalent to calling <a href="http://doc.qt.io/qt-5/qlist.html#removeAt">removeAt</a>(0). The list must not be empty. If the list can be empty, call <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() before calling this function.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#removeAt">removeAt</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#takeFirst">takeFirst</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_removeFirst(
        this_ptr: *mut crate::QListOfQWebEngineScript,
    );

    /// <p>Removes the last item in the list. Calling this function is equivalent to calling <a href="http://doc.qt.io/qt-5/qlist.html#removeAt">removeAt</a>(<a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>() - 1). The list must not be empty. If the list can be empty, call <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() before calling this function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#removeLast">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Removes the last item in the list. Calling this function is equivalent to calling <a href="http://doc.qt.io/qt-5/qlist.html#removeAt">removeAt</a>(<a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>() - 1). The list must not be empty. If the list can be empty, call <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() before calling this function.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#removeAt">removeAt</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#takeLast">takeLast</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_removeLast(
        this_ptr: *mut crate::QListOfQWebEngineHistoryItem,
    );

    /// <p>Removes the last item in the list. Calling this function is equivalent to calling <a href="http://doc.qt.io/qt-5/qlist.html#removeAt">removeAt</a>(<a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>() - 1). The list must not be empty. If the list can be empty, call <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() before calling this function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#removeLast">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Removes the last item in the list. Calling this function is equivalent to calling <a href="http://doc.qt.io/qt-5/qlist.html#removeAt">removeAt</a>(<a href="http://doc.qt.io/qt-5/qlist.html#size">size</a>() - 1). The list must not be empty. If the list can be empty, call <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() before calling this function.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#removeAt">removeAt</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#takeLast">takeLast</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_removeLast(
        this_ptr: *mut crate::QListOfQWebEngineScript,
    );

    /// <p>Returns <code>true</code> if this list is not empty and its first item is equal to <i>value</i>; otherwise returns <code>false</code>.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#startsWith">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if this list is not empty and its first item is equal to <i>value</i>; otherwise returns <code>false</code>.</p>
    /// <p>This function was introduced in Qt 4.5.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#contains">contains</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_startsWith(
        this_ptr: *const crate::QListOfQWebEngineScript,
        t: *const crate::QWebEngineScript,
    ) -> bool;

    /// <p>Returns <code>true</code> if this list is not empty and its last item is equal to <i>value</i>; otherwise returns <code>false</code>.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#endsWith">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if this list is not empty and its last item is equal to <i>value</i>; otherwise returns <code>false</code>.</p>
    /// <p>This function was introduced in Qt 4.5.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#contains">contains</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_endsWith(
        this_ptr: *const crate::QListOfQWebEngineScript,
        t: *const crate::QWebEngineScript,
    ) -> bool;

    /// <p>Returns a sub-list which includes elements from this list, starting at position <i>pos</i>. If <i>length</i> is -1 (the default), all elements from <i>pos</i> are included; otherwise <i>length</i> elements (or all remaining elements if there are less than <i>length</i> elements) are included.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#mid">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns a sub-list which includes elements from this list, starting at position <i>pos</i>. If <i>length</i> is -1 (the default), all elements from <i>pos</i> are included; otherwise <i>length</i> elements (or all remaining elements if there are less than <i>length</i> elements) are included.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_mid(
        this_ptr: *const crate::QListOfQWebEngineHistoryItem,
        pos: ::std::os::raw::c_int,
        length: ::std::os::raw::c_int,
    ) -> *mut crate::QListOfQWebEngineHistoryItem;

    /// <p>Returns a sub-list which includes elements from this list, starting at position <i>pos</i>. If <i>length</i> is -1 (the default), all elements from <i>pos</i> are included; otherwise <i>length</i> elements (or all remaining elements if there are less than <i>length</i> elements) are included.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#mid">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns a sub-list which includes elements from this list, starting at position <i>pos</i>. If <i>length</i> is -1 (the default), all elements from <i>pos</i> are included; otherwise <i>length</i> elements (or all remaining elements if there are less than <i>length</i> elements) are included.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_mid(
        this_ptr: *const crate::QListOfQWebEngineScript,
        pos: ::std::os::raw::c_int,
        length: ::std::os::raw::c_int,
    ) -> *mut crate::QListOfQWebEngineScript;

    /// <p>Returns the value at index position <i>i</i> in the list.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#value">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns the value at index position <i>i</i> in the list.</p>
    /// <p>If the index <i>i</i> is out of bounds, the function returns a <a href="http://doc.qt.io/qt-5/containers.html#default-constructed-value">default-constructed value</a>. If you are certain that the index is going to be within bounds, you can use <a href="http://doc.qt.io/qt-5/qlist.html#at">at</a>() instead, which is slightly faster.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#at">at</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#operator-5b-5d">operator[]</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_value(
        this_ptr: *const crate::QListOfQWebEngineScript,
        i: ::std::os::raw::c_int,
    ) -> *mut crate::QWebEngineScript;

    /// <p>This is an overloaded function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#value-1">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This is an overloaded function.</p>
    /// <p>If the index <i>i</i> is out of bounds, the function returns <i>defaultValue</i>.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_value1(
        this_ptr: *const crate::QListOfQWebEngineHistoryItem,
        i: ::std::os::raw::c_int,
        defaultValue: *const crate::QWebEngineHistoryItem,
    ) -> *mut crate::QWebEngineHistoryItem;

    /// <p>This is an overloaded function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#value-1">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This is an overloaded function.</p>
    /// <p>If the index <i>i</i> is out of bounds, the function returns <i>defaultValue</i>.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_value1(
        this_ptr: *const crate::QListOfQWebEngineScript,
        i: ::std::os::raw::c_int,
        defaultValue: *const crate::QWebEngineScript,
    ) -> *mut crate::QWebEngineScript;

    /// <p>This function is provided for STL compatibility. It is equivalent to <a href="http://doc.qt.io/qt-5/qlist.html#append">append</a>(<i>value</i>).</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#push_back">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This function is provided for STL compatibility. It is equivalent to <a href="http://doc.qt.io/qt-5/qlist.html#append">append</a>(<i>value</i>).</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_push_back(
        this_ptr: *mut crate::QListOfQWebEngineHistoryItem,
        t: *const crate::QWebEngineHistoryItem,
    );

    /// <p>This function is provided for STL compatibility. It is equivalent to <a href="http://doc.qt.io/qt-5/qlist.html#append">append</a>(<i>value</i>).</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#push_back">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This function is provided for STL compatibility. It is equivalent to <a href="http://doc.qt.io/qt-5/qlist.html#append">append</a>(<i>value</i>).</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_push_back(
        this_ptr: *mut crate::QListOfQWebEngineScript,
        t: *const crate::QWebEngineScript,
    );

    /// <p>This function is provided for STL compatibility. It is equivalent to <a href="http://doc.qt.io/qt-5/qlist.html#prepend">prepend</a>(<i>value</i>).</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#push_front">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This function is provided for STL compatibility. It is equivalent to <a href="http://doc.qt.io/qt-5/qlist.html#prepend">prepend</a>(<i>value</i>).</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_push_front(
        this_ptr: *mut crate::QListOfQWebEngineHistoryItem,
        t: *const crate::QWebEngineHistoryItem,
    );

    /// <p>This function is provided for STL compatibility. It is equivalent to <a href="http://doc.qt.io/qt-5/qlist.html#prepend">prepend</a>(<i>value</i>).</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#push_front">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This function is provided for STL compatibility. It is equivalent to <a href="http://doc.qt.io/qt-5/qlist.html#prepend">prepend</a>(<i>value</i>).</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_push_front(
        this_ptr: *mut crate::QListOfQWebEngineScript,
        t: *const crate::QWebEngineScript,
    );

    /// <p>This function is provided for STL compatibility. It is equivalent to <a href="http://doc.qt.io/qt-5/qlist.html#first">first</a>(). The list must not be empty. If the list can be empty, call <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() before calling this function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#front">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This function is provided for STL compatibility. It is equivalent to <a href="http://doc.qt.io/qt-5/qlist.html#first">first</a>(). The list must not be empty. If the list can be empty, call <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() before calling this function.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_front(
        this_ptr: *mut crate::QListOfQWebEngineHistoryItem,
    ) -> *mut crate::QWebEngineHistoryItem;

    /// <p>This function is provided for STL compatibility. It is equivalent to <a href="http://doc.qt.io/qt-5/qlist.html#first">first</a>(). The list must not be empty. If the list can be empty, call <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() before calling this function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#front">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This function is provided for STL compatibility. It is equivalent to <a href="http://doc.qt.io/qt-5/qlist.html#first">first</a>(). The list must not be empty. If the list can be empty, call <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() before calling this function.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_front(
        this_ptr: *mut crate::QListOfQWebEngineScript,
    ) -> *mut crate::QWebEngineScript;

    /// <p>This is an overloaded function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#front-1">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This is an overloaded function.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_front1(
        this_ptr: *const crate::QListOfQWebEngineHistoryItem,
    ) -> *const crate::QWebEngineHistoryItem;

    /// <p>This is an overloaded function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#front-1">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This is an overloaded function.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_front1(
        this_ptr: *const crate::QListOfQWebEngineScript,
    ) -> *const crate::QWebEngineScript;

    /// <p>This function is provided for STL compatibility. It is equivalent to <a href="http://doc.qt.io/qt-5/qlist.html#last">last</a>(). The list must not be empty. If the list can be empty, call <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() before calling this function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#back">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This function is provided for STL compatibility. It is equivalent to <a href="http://doc.qt.io/qt-5/qlist.html#last">last</a>(). The list must not be empty. If the list can be empty, call <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() before calling this function.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_back(
        this_ptr: *mut crate::QListOfQWebEngineHistoryItem,
    ) -> *mut crate::QWebEngineHistoryItem;

    /// <p>This function is provided for STL compatibility. It is equivalent to <a href="http://doc.qt.io/qt-5/qlist.html#last">last</a>(). The list must not be empty. If the list can be empty, call <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() before calling this function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#back">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This function is provided for STL compatibility. It is equivalent to <a href="http://doc.qt.io/qt-5/qlist.html#last">last</a>(). The list must not be empty. If the list can be empty, call <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() before calling this function.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_back(
        this_ptr: *mut crate::QListOfQWebEngineScript,
    ) -> *mut crate::QWebEngineScript;

    /// <p>This is an overloaded function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#back-1">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This is an overloaded function.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_back1(
        this_ptr: *const crate::QListOfQWebEngineHistoryItem,
    ) -> *const crate::QWebEngineHistoryItem;

    /// <p>This is an overloaded function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#back-1">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This is an overloaded function.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_back1(
        this_ptr: *const crate::QListOfQWebEngineScript,
    ) -> *const crate::QWebEngineScript;

    /// <p>This function is provided for STL compatibility. It is equivalent to <a href="http://doc.qt.io/qt-5/qlist.html#removeFirst">removeFirst</a>(). The list must not be empty. If the list can be empty, call <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() before calling this function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#pop_front">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This function is provided for STL compatibility. It is equivalent to <a href="http://doc.qt.io/qt-5/qlist.html#removeFirst">removeFirst</a>(). The list must not be empty. If the list can be empty, call <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() before calling this function.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_pop_front(
        this_ptr: *mut crate::QListOfQWebEngineHistoryItem,
    );

    /// <p>This function is provided for STL compatibility. It is equivalent to <a href="http://doc.qt.io/qt-5/qlist.html#removeFirst">removeFirst</a>(). The list must not be empty. If the list can be empty, call <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() before calling this function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#pop_front">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This function is provided for STL compatibility. It is equivalent to <a href="http://doc.qt.io/qt-5/qlist.html#removeFirst">removeFirst</a>(). The list must not be empty. If the list can be empty, call <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() before calling this function.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_pop_front(
        this_ptr: *mut crate::QListOfQWebEngineScript,
    );

    /// <p>This function is provided for STL compatibility. It is equivalent to <a href="http://doc.qt.io/qt-5/qlist.html#removeLast">removeLast</a>(). The list must not be empty. If the list can be empty, call <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() before calling this function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#pop_back">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This function is provided for STL compatibility. It is equivalent to <a href="http://doc.qt.io/qt-5/qlist.html#removeLast">removeLast</a>(). The list must not be empty. If the list can be empty, call <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() before calling this function.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_pop_back(
        this_ptr: *mut crate::QListOfQWebEngineHistoryItem,
    );

    /// <p>This function is provided for STL compatibility. It is equivalent to <a href="http://doc.qt.io/qt-5/qlist.html#removeLast">removeLast</a>(). The list must not be empty. If the list can be empty, call <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() before calling this function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#pop_back">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This function is provided for STL compatibility. It is equivalent to <a href="http://doc.qt.io/qt-5/qlist.html#removeLast">removeLast</a>(). The list must not be empty. If the list can be empty, call <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() before calling this function.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_pop_back(
        this_ptr: *mut crate::QListOfQWebEngineScript,
    );

    /// <p>This function is provided for STL compatibility. It is equivalent to <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() and returns <code>true</code> if the list is empty.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#empty">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This function is provided for STL compatibility. It is equivalent to <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() and returns <code>true</code> if the list is empty.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_empty(
        this_ptr: *const crate::QListOfQWebEngineHistoryItem,
    ) -> bool;

    /// <p>This function is provided for STL compatibility. It is equivalent to <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() and returns <code>true</code> if the list is empty.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#empty">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This function is provided for STL compatibility. It is equivalent to <a href="http://doc.qt.io/qt-5/qlist.html#isEmpty">isEmpty</a>() and returns <code>true</code> if the list is empty.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_empty(
        this_ptr: *const crate::QListOfQWebEngineScript,
    ) -> bool;

    /// <p>Appends the items of the <i>other</i> list to this list and returns a reference to this list.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#operator-2b-eq">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Appends the items of the <i>other</i> list to this list and returns a reference to this list.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#operator-2b">operator+</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#append">append</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_operator__4(
        this_ptr: *mut crate::QListOfQWebEngineHistoryItem,
        l: *const crate::QListOfQWebEngineHistoryItem,
    ) -> *mut crate::QListOfQWebEngineHistoryItem;

    /// <p>Appends the items of the <i>other</i> list to this list and returns a reference to this list.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#operator-2b-eq">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Appends the items of the <i>other</i> list to this list and returns a reference to this list.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#operator-2b">operator+</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#append">append</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_operator__4(
        this_ptr: *mut crate::QListOfQWebEngineScript,
        l: *const crate::QListOfQWebEngineScript,
    ) -> *mut crate::QListOfQWebEngineScript;

    /// <p>Returns a list that contains all the items in this list followed by all the items in the <i>other</i> list.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#operator-2b">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns a list that contains all the items in this list followed by all the items in the <i>other</i> list.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#operator-2b-eq">operator+=</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_operator_2(
        this_ptr: *const crate::QListOfQWebEngineHistoryItem,
        l: *const crate::QListOfQWebEngineHistoryItem,
    ) -> *mut crate::QListOfQWebEngineHistoryItem;

    /// <p>Returns a list that contains all the items in this list followed by all the items in the <i>other</i> list.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#operator-2b">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns a list that contains all the items in this list followed by all the items in the <i>other</i> list.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#operator-2b-eq">operator+=</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_operator_2(
        this_ptr: *const crate::QListOfQWebEngineScript,
        l: *const crate::QListOfQWebEngineScript,
    ) -> *mut crate::QListOfQWebEngineScript;

    /// <p>This is an overloaded function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#operator-2b-eq-1">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This is an overloaded function.</p>
    /// <p>Appends <i>value</i> to the list.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#append">append</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#operator-lt-lt">operator&lt;&lt;</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_operator__5(
        this_ptr: *mut crate::QListOfQWebEngineHistoryItem,
        t: *const crate::QWebEngineHistoryItem,
    ) -> *mut crate::QListOfQWebEngineHistoryItem;

    /// <p>This is an overloaded function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#operator-2b-eq-1">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This is an overloaded function.</p>
    /// <p>Appends <i>value</i> to the list.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#append">append</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#operator-lt-lt">operator&lt;&lt;</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_operator__5(
        this_ptr: *mut crate::QListOfQWebEngineScript,
        t: *const crate::QWebEngineScript,
    ) -> *mut crate::QListOfQWebEngineScript;

    /// <p>This is an overloaded function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#operator-lt-lt-1">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This is an overloaded function.</p>
    /// <p>Appends <i>value</i> to the list.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_operator__6(
        this_ptr: *mut crate::QListOfQWebEngineHistoryItem,
        t: *const crate::QWebEngineHistoryItem,
    ) -> *mut crate::QListOfQWebEngineHistoryItem;

    /// <p>This is an overloaded function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#operator-lt-lt-1">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This is an overloaded function.</p>
    /// <p>Appends <i>value</i> to the list.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_operator__6(
        this_ptr: *mut crate::QListOfQWebEngineScript,
        t: *const crate::QWebEngineScript,
    ) -> *mut crate::QListOfQWebEngineScript;

    /// <p>Appends the items of the <i>other</i> list to this list and returns a reference to this list.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#operator-lt-lt">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Appends the items of the <i>other</i> list to this list and returns a reference to this list.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#operator-2b-eq">operator+=</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#append">append</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_operator__7(
        this_ptr: *mut crate::QListOfQWebEngineHistoryItem,
        l: *const crate::QListOfQWebEngineHistoryItem,
    ) -> *mut crate::QListOfQWebEngineHistoryItem;

    /// <p>Appends the items of the <i>other</i> list to this list and returns a reference to this list.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#operator-lt-lt">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Appends the items of the <i>other</i> list to this list and returns a reference to this list.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#operator-2b-eq">operator+=</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#append">append</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_operator__7(
        this_ptr: *mut crate::QListOfQWebEngineScript,
        l: *const crate::QListOfQWebEngineScript,
    ) -> *mut crate::QListOfQWebEngineScript;

    /// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
    ///
    /// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_operator__13(
        debug: *const ::qt_core::QDebug,
        list: *const crate::QListOfQWebEngineScript,
    ) -> *mut ::qt_core::QDebug;

    /// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
    ///
    /// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_operator__15(
        debug: *const ::qt_core::QDebug,
        flags: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QDebug;

    /// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
    ///
    /// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_operator__16(
        debug: *const ::qt_core::QDebug,
        flags: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QDebug;

    /// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
    ///
    /// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_operator__17(
        debug: *const ::qt_core::QDebug,
        flags: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QDebug;

    /// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
    ///
    /// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_operator__18(
        s: *mut ::qt_core::QDataStream,
        e: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QDataStream;

    /// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
    ///
    /// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_operator__19(
        s: *mut ::qt_core::QDataStream,
        e: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QDataStream;

    /// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
    ///
    /// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_operator__20(
        s: *mut ::qt_core::QDataStream,
        e: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QDataStream;

    /// <p>Constructs a guarded pointer with value <code>nullptr</code>.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qpointer.html#QPointer">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Constructs a guarded pointer with value <code>nullptr</code>.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qpointer.html#isNull">isNull</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QPointer_QWebEnginePage_QPointer(
    ) -> *mut crate::QPointerOfQWebEnginePage;

    /// <p>Constructs a guarded pointer that points to the same object that <i>p</i> points to.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qpointer.html#QPointer-1">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Constructs a guarded pointer that points to the same object that <i>p</i> points to.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QPointer_QWebEnginePage_QPointer1(
        p: *mut crate::QWebEnginePage,
    ) -> *mut crate::QPointerOfQWebEnginePage;

    /// <p>Swaps the contents of this <a href="http://doc.qt.io/qt-5/qpointer.html">QPointer</a> with the contents of <i>other</i>. This operation is very fast and never fails.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qpointer.html#swap">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Swaps the contents of this <a href="http://doc.qt.io/qt-5/qpointer.html">QPointer</a> with the contents of <i>other</i>. This operation is very fast and never fails.</p>
    /// <p>This function was introduced in Qt 5.6.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QPointer_QWebEnginePage_swap(
        this_ptr: *mut crate::QPointerOfQWebEnginePage,
        other: *mut crate::QPointerOfQWebEnginePage,
    );

    /// <p>Assignment operator. This guarded pointer will now point to the same object that <i>p</i> points to.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qpointer.html#operator-eq">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Assignment operator. This guarded pointer will now point to the same object that <i>p</i> points to.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QPointer_QWebEnginePage_operator_(
        this_ptr: *mut crate::QPointerOfQWebEnginePage,
        p: *mut crate::QWebEnginePage,
    ) -> *mut crate::QPointerOfQWebEnginePage;

    /// <p>Returns the pointer to the object being guarded.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qpointer.html#data">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns the pointer to the object being guarded.</p>
    /// <p>This function was introduced in Qt 4.4.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QPointer_QWebEnginePage_data(
        this_ptr: *const crate::QPointerOfQWebEnginePage,
    ) -> *mut crate::QWebEnginePage;

    /// <p>Overloaded arrow operator; implements pointer semantics. Just use this operator as you would with a normal C++ pointer.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qpointer.html#operator--gt">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Overloaded arrow operator; implements pointer semantics. Just use this operator as you would with a normal C++ pointer.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QPointer_QWebEnginePage_operator__(
        this_ptr: *const crate::QPointerOfQWebEnginePage,
    ) -> *mut crate::QWebEnginePage;

    /// <p>Dereference operator; implements pointer semantics. Just use this operator as you would with a normal C++ pointer.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qpointer.html#operator-2a">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Dereference operator; implements pointer semantics. Just use this operator as you would with a normal C++ pointer.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QPointer_QWebEnginePage_operator_1(
        this_ptr: *const crate::QPointerOfQWebEnginePage,
    ) -> *mut crate::QWebEnginePage;

    pub fn ctr_qt_web_engine_widgets_ffi_QPointer_QWebEnginePage_operator_QWebEnginePage__(
        this_ptr: *const crate::QPointerOfQWebEnginePage,
    ) -> *mut crate::QWebEnginePage;

    /// <p>Returns <code>true</code> if the referenced object has been destroyed or if there is no referenced object; otherwise returns <code>false</code>.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qpointer.html#isNull">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if the referenced object has been destroyed or if there is no referenced object; otherwise returns <code>false</code>.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QPointer_QWebEnginePage_isNull(
        this_ptr: *const crate::QPointerOfQWebEnginePage,
    ) -> bool;

    /// <p>Clears this <a href="http://doc.qt.io/qt-5/qpointer.html">QPointer</a> object.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qpointer.html#clear">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Clears this <a href="http://doc.qt.io/qt-5/qpointer.html">QPointer</a> object.</p>
    /// <p>This function was introduced in Qt 5.0.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qpointer.html#isNull">isNull</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QPointer_QWebEnginePage_clear(
        this_ptr: *mut crate::QPointerOfQWebEnginePage,
    );

    /// <p>Returns <code>true</code> if <i>lhs</i> and <i>rhs</i> are equal, otherwise returns <code>false</code>.</p>
    ///
    /// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbytearray-frombase64result.html#operator-eq-eq">C++ documentation</a> for <span style='color: green;'>```bool operator==(const QByteArray::FromBase64Result &lhs, const QByteArray::FromBase64Result &rhs)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if <i>lhs</i> and <i>rhs</i> are equal, otherwise returns <code>false</code>.</p>
    /// <p><i>lhs</i> and <i>rhs</i> are equal if and only if they contain the same decoding status and, if the status is QByteArray::Base64DecodingStatus::Ok, if and only if they contain the same decoded data.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_operator__32(
        o: *const crate::QWebEnginePage,
        p: *const crate::QPointerOfQWebEnginePage,
    ) -> bool;

    /// <p>Returns <code>true</code> if <i>lhs</i> and <i>rhs</i> are equal, otherwise returns <code>false</code>.</p>
    ///
    /// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbytearray-frombase64result.html#operator-eq-eq">C++ documentation</a> for <span style='color: green;'>```bool operator==(const QByteArray::FromBase64Result &lhs, const QByteArray::FromBase64Result &rhs)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if <i>lhs</i> and <i>rhs</i> are equal, otherwise returns <code>false</code>.</p>
    /// <p><i>lhs</i> and <i>rhs</i> are equal if and only if they contain the same decoding status and, if the status is QByteArray::Base64DecodingStatus::Ok, if and only if they contain the same decoded data.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_operator__33(
        p: *const crate::QPointerOfQWebEnginePage,
        o: *const crate::QWebEnginePage,
    ) -> bool;

    /// <p>Returns <code>true</code> if <i>lhs</i> and <i>rhs</i> are equal, otherwise returns <code>false</code>.</p>
    ///
    /// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbytearray-frombase64result.html#operator-eq-eq">C++ documentation</a> for <span style='color: green;'>```bool operator==(const QByteArray::FromBase64Result &lhs, const QByteArray::FromBase64Result &rhs)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if <i>lhs</i> and <i>rhs</i> are equal, otherwise returns <code>false</code>.</p>
    /// <p><i>lhs</i> and <i>rhs</i> are equal if and only if they contain the same decoding status and, if the status is QByteArray::Base64DecodingStatus::Ok, if and only if they contain the same decoded data.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_operator__34(
        o: *mut crate::QWebEnginePage,
        p: *const crate::QPointerOfQWebEnginePage,
    ) -> bool;

    /// <p>Returns <code>true</code> if <i>lhs</i> and <i>rhs</i> are equal, otherwise returns <code>false</code>.</p>
    ///
    /// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbytearray-frombase64result.html#operator-eq-eq">C++ documentation</a> for <span style='color: green;'>```bool operator==(const QByteArray::FromBase64Result &lhs, const QByteArray::FromBase64Result &rhs)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if <i>lhs</i> and <i>rhs</i> are equal, otherwise returns <code>false</code>.</p>
    /// <p><i>lhs</i> and <i>rhs</i> are equal if and only if they contain the same decoding status and, if the status is QByteArray::Base64DecodingStatus::Ok, if and only if they contain the same decoded data.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_operator__35(
        p: *const crate::QPointerOfQWebEnginePage,
        o: *mut crate::QWebEnginePage,
    ) -> bool;

    /// <p>Returns <code>true</code> if <i>lhs</i> and <i>rhs</i> are equal, otherwise returns <code>false</code>.</p>
    ///
    /// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbytearray-frombase64result.html#operator-eq-eq">C++ documentation</a> for <span style='color: green;'>```bool operator==(const QByteArray::FromBase64Result &lhs, const QByteArray::FromBase64Result &rhs)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if <i>lhs</i> and <i>rhs</i> are equal, otherwise returns <code>false</code>.</p>
    /// <p><i>lhs</i> and <i>rhs</i> are equal if and only if they contain the same decoding status and, if the status is QByteArray::Base64DecodingStatus::Ok, if and only if they contain the same decoded data.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_operator__36(
        p1: *const crate::QPointerOfQWebEnginePage,
        p2: *const crate::QPointerOfQWebEnginePage,
    ) -> bool;

    pub fn ctr_qt_web_engine_widgets_ffi_qPointerFromVariant_QWebEnginePage(
        variant: *const ::qt_core::QVariant,
    ) -> *mut crate::QPointerOfQWebEnginePage;

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_MemoryLayout_dMemoryLayout(
        this_ptr: *mut crate::q_list_of_q_web_engine_history_item::MemoryLayout,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_MemoryLayout_dMemoryLayout(
        this_ptr: *mut crate::q_list_of_q_web_engine_script::MemoryLayout,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_MemoryLayout_MemoryLayout(
    ) -> *mut crate::q_list_of_q_web_engine_history_item::MemoryLayout;

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_MemoryLayout_MemoryLayout(
    ) -> *mut crate::q_list_of_q_web_engine_script::MemoryLayout;

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_MemoryLayout_MemoryLayout1(
        other: *const crate::q_list_of_q_web_engine_history_item::MemoryLayout,
    ) -> *mut crate::q_list_of_q_web_engine_history_item::MemoryLayout;

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_MemoryLayout_MemoryLayout1(
        other: *const crate::q_list_of_q_web_engine_script::MemoryLayout,
    ) -> *mut crate::q_list_of_q_web_engine_script::MemoryLayout;

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_MemoryLayout_operator_(
        this_ptr: *mut crate::q_list_of_q_web_engine_history_item::MemoryLayout,
        other: *const crate::q_list_of_q_web_engine_history_item::MemoryLayout,
    ) -> *mut crate::q_list_of_q_web_engine_history_item::MemoryLayout;

    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_MemoryLayout_operator_(
        this_ptr: *mut crate::q_list_of_q_web_engine_script::MemoryLayout,
        other: *const crate::q_list_of_q_web_engine_script::MemoryLayout,
    ) -> *mut crate::q_list_of_q_web_engine_script::MemoryLayout;

    /// <p><a href="http://doc.qt.io/qt-5/qlist.html">QList</a> features both <a href="http://doc.qt.io/qt-5/containers.html#stl-style-iterators">STL-style iterators</a> and <a href="http://doc.qt.io/qt-5/containers.html#java-style-iterators">Java-style iterators</a>. The STL-style iterators are more low-level and more cumbersome to use; on the other hand, they are slightly faster and, for developers who already know STL, have the advantage of familiarity.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist-iterator.html">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p><a href="http://doc.qt.io/qt-5/qlist.html">QList</a> features both <a href="http://doc.qt.io/qt-5/containers.html#stl-style-iterators">STL-style iterators</a> and <a href="http://doc.qt.io/qt-5/containers.html#java-style-iterators">Java-style iterators</a>. The STL-style iterators are more low-level and more cumbersome to use; on the other hand, they are slightly faster and, for developers who already know STL, have the advantage of familiarity.</p>
    /// <p><a href="http://doc.qt.io/qt-5/qlist.html">QList</a>&lt;T&gt;::iterator allows you to iterate over a <a href="http://doc.qt.io/qt-5/qlist.html">QList</a>&lt;T&gt; (or <a href="http://doc.qt.io/qt-5/qqueue.html">QQueue</a>&lt;T&gt;) and to modify the list item associated with the iterator. If you want to iterate over a const <a href="http://doc.qt.io/qt-5/qlist.html">QList</a>, use <a href="http://doc.qt.io/qt-5/qlist-const-iterator.html">QList::const_iterator</a> instead. It is generally good practice to use <a href="http://doc.qt.io/qt-5/qlist-const-iterator.html">QList::const_iterator</a> on a non-const <a href="http://doc.qt.io/qt-5/qlist.html">QList</a> as well, unless you need to change the <a href="http://doc.qt.io/qt-5/qlist.html">QList</a> through the iterator. Const iterators are slightly faster, and can improve code readability.</p>
    /// <p>The default <a href="http://doc.qt.io/qt-5/qlist-iterator.html">QList::iterator</a> constructor creates an uninitialized iterator. You must initialize it using a <a href="http://doc.qt.io/qt-5/qlist.html">QList</a> function like <a href="http://doc.qt.io/qt-5/qlist.html#begin">QList::begin</a>(), <a href="http://doc.qt.io/qt-5/qlist.html#end">QList::end</a>(), or <a href="http://doc.qt.io/qt-5/qlist.html#insert">QList::insert</a>() before you can start iterating. Here's a typical loop that prints all the items stored in a list:</p>
    /// <pre class="cpp"> <span class="type"><a href="http://doc.qt.io/qt-5/qlist.html#QList">QList</a></span><span class="operator">&lt;</span><span class="type"><a href="http://doc.qt.io/qt-5/qstring.html">QString</a></span><span class="operator">&gt;</span> list;
    ///  list<span class="operator">.</span>append(<span class="string">"January"</span>);
    ///  list<span class="operator">.</span>append(<span class="string">"February"</span>);
    ///  <span class="operator">.</span><span class="operator">.</span><span class="operator">.</span>
    ///  list<span class="operator">.</span>append(<span class="string">"December"</span>);
    ///
    ///  <span class="type"><a href="http://doc.qt.io/qt-5/qlist.html#QList">QList</a></span><span class="operator">&lt;</span><span class="type"><a href="http://doc.qt.io/qt-5/qstring.html">QString</a></span><span class="operator">&gt;</span><span class="operator">::</span>iterator i;
    ///  <span class="keyword">for</span> (i <span class="operator">=</span> list<span class="operator">.</span>begin(); i <span class="operator">!</span><span class="operator">=</span> list<span class="operator">.</span>end(); <span class="operator">+</span><span class="operator">+</span>i)
    /// &#32;    cout <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="operator">*</span>i <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="type">Qt</span><span class="operator">::</span>endl;
    /// </pre>
    /// <p>Let's see a few examples of things we can do with a <a href="http://doc.qt.io/qt-5/qlist-iterator.html">QList::iterator</a> that we cannot do with a <a href="http://doc.qt.io/qt-5/qlist-const-iterator.html">QList::const_iterator</a>. Here's an example that increments every value stored in a <a href="http://doc.qt.io/qt-5/qlist.html">QList</a>&lt;int&gt; by 2:</p>
    /// <pre class="cpp"> <span class="type"><a href="http://doc.qt.io/qt-5/qlist.html#QList">QList</a></span><span class="operator">&lt;</span><span class="type">int</span><span class="operator">&gt;</span><span class="operator">::</span>iterator i;
    ///  <span class="keyword">for</span> (i <span class="operator">=</span> list<span class="operator">.</span>begin(); i <span class="operator">!</span><span class="operator">=</span> list<span class="operator">.</span>end(); <span class="operator">+</span><span class="operator">+</span>i)
    /// &#32;    <span class="operator">*</span>i <span class="operator">+</span><span class="operator">=</span> <span class="number">2</span>;
    /// </pre>
    /// <p>Most <a href="http://doc.qt.io/qt-5/qlist.html">QList</a> functions accept an integer index rather than an iterator. For that reason, iterators are rarely useful in connection with <a href="http://doc.qt.io/qt-5/qlist.html">QList</a>. One place where STL-style iterators do make sense is as arguments to <a href="http://doc.qt.io/qt-5/qtalgorithms.html">generic algorithms</a>.</p>
    /// <p>For example, here's how to delete all the widgets stored in a <a href="http://doc.qt.io/qt-5/qlist.html">QList</a>&lt;<a href="http://doc.qt.io/qt-5/qwidget.html">QWidget</a> *&gt;:</p>
    /// <pre class="cpp"> <span class="type"><a href="http://doc.qt.io/qt-5/qlist.html#QList">QList</a></span><span class="operator">&lt;</span><span class="type"><a href="http://doc.qt.io/qt-5/qwidget.html">QWidget</a></span> <span class="operator">*</span><span class="operator">&gt;</span> list;
    ///  <span class="operator">.</span><span class="operator">.</span><span class="operator">.</span>
    ///  <a href="http://doc.qt.io/qt-5/qtalgorithms.html#qDeleteAll">qDeleteAll</a>(list<span class="operator">.</span>begin()<span class="operator">,</span> list<span class="operator">.</span>end());
    /// </pre>
    /// <p>Multiple iterators can be used on the same list. However, be aware that any non-const function call performed on the <a href="http://doc.qt.io/qt-5/qlist.html">QList</a> will render all existing iterators undefined. If you need to keep iterators over a long period of time, we recommend that you use QLinkedList rather than <a href="http://doc.qt.io/qt-5/qlist.html">QList</a>.</p>
    /// <p><b>Warning:</b> Iterators on implicitly shared containers do not work exactly like STL-iterators. You should avoid copying a container while iterators are active on that container. For more information, read <a href="http://doc.qt.io/qt-5/containers.html#implicit-sharing-iterator-problem">Implicit sharing iterator problem</a>.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_iterator_diterator(
        this_ptr: *mut crate::q_list_of_q_web_engine_history_item::Iterator,
    );

    /// <p><a href="http://doc.qt.io/qt-5/qlist.html">QList</a> features both <a href="http://doc.qt.io/qt-5/containers.html#stl-style-iterators">STL-style iterators</a> and <a href="http://doc.qt.io/qt-5/containers.html#java-style-iterators">Java-style iterators</a>. The STL-style iterators are more low-level and more cumbersome to use; on the other hand, they are slightly faster and, for developers who already know STL, have the advantage of familiarity.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist-iterator.html">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p><a href="http://doc.qt.io/qt-5/qlist.html">QList</a> features both <a href="http://doc.qt.io/qt-5/containers.html#stl-style-iterators">STL-style iterators</a> and <a href="http://doc.qt.io/qt-5/containers.html#java-style-iterators">Java-style iterators</a>. The STL-style iterators are more low-level and more cumbersome to use; on the other hand, they are slightly faster and, for developers who already know STL, have the advantage of familiarity.</p>
    /// <p><a href="http://doc.qt.io/qt-5/qlist.html">QList</a>&lt;T&gt;::iterator allows you to iterate over a <a href="http://doc.qt.io/qt-5/qlist.html">QList</a>&lt;T&gt; (or <a href="http://doc.qt.io/qt-5/qqueue.html">QQueue</a>&lt;T&gt;) and to modify the list item associated with the iterator. If you want to iterate over a const <a href="http://doc.qt.io/qt-5/qlist.html">QList</a>, use <a href="http://doc.qt.io/qt-5/qlist-const-iterator.html">QList::const_iterator</a> instead. It is generally good practice to use <a href="http://doc.qt.io/qt-5/qlist-const-iterator.html">QList::const_iterator</a> on a non-const <a href="http://doc.qt.io/qt-5/qlist.html">QList</a> as well, unless you need to change the <a href="http://doc.qt.io/qt-5/qlist.html">QList</a> through the iterator. Const iterators are slightly faster, and can improve code readability.</p>
    /// <p>The default <a href="http://doc.qt.io/qt-5/qlist-iterator.html">QList::iterator</a> constructor creates an uninitialized iterator. You must initialize it using a <a href="http://doc.qt.io/qt-5/qlist.html">QList</a> function like <a href="http://doc.qt.io/qt-5/qlist.html#begin">QList::begin</a>(), <a href="http://doc.qt.io/qt-5/qlist.html#end">QList::end</a>(), or <a href="http://doc.qt.io/qt-5/qlist.html#insert">QList::insert</a>() before you can start iterating. Here's a typical loop that prints all the items stored in a list:</p>
    /// <pre class="cpp"> <span class="type"><a href="http://doc.qt.io/qt-5/qlist.html#QList">QList</a></span><span class="operator">&lt;</span><span class="type"><a href="http://doc.qt.io/qt-5/qstring.html">QString</a></span><span class="operator">&gt;</span> list;
    ///  list<span class="operator">.</span>append(<span class="string">"January"</span>);
    ///  list<span class="operator">.</span>append(<span class="string">"February"</span>);
    ///  <span class="operator">.</span><span class="operator">.</span><span class="operator">.</span>
    ///  list<span class="operator">.</span>append(<span class="string">"December"</span>);
    ///
    ///  <span class="type"><a href="http://doc.qt.io/qt-5/qlist.html#QList">QList</a></span><span class="operator">&lt;</span><span class="type"><a href="http://doc.qt.io/qt-5/qstring.html">QString</a></span><span class="operator">&gt;</span><span class="operator">::</span>iterator i;
    ///  <span class="keyword">for</span> (i <span class="operator">=</span> list<span class="operator">.</span>begin(); i <span class="operator">!</span><span class="operator">=</span> list<span class="operator">.</span>end(); <span class="operator">+</span><span class="operator">+</span>i)
    /// &#32;    cout <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="operator">*</span>i <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="type">Qt</span><span class="operator">::</span>endl;
    /// </pre>
    /// <p>Let's see a few examples of things we can do with a <a href="http://doc.qt.io/qt-5/qlist-iterator.html">QList::iterator</a> that we cannot do with a <a href="http://doc.qt.io/qt-5/qlist-const-iterator.html">QList::const_iterator</a>. Here's an example that increments every value stored in a <a href="http://doc.qt.io/qt-5/qlist.html">QList</a>&lt;int&gt; by 2:</p>
    /// <pre class="cpp"> <span class="type"><a href="http://doc.qt.io/qt-5/qlist.html#QList">QList</a></span><span class="operator">&lt;</span><span class="type">int</span><span class="operator">&gt;</span><span class="operator">::</span>iterator i;
    ///  <span class="keyword">for</span> (i <span class="operator">=</span> list<span class="operator">.</span>begin(); i <span class="operator">!</span><span class="operator">=</span> list<span class="operator">.</span>end(); <span class="operator">+</span><span class="operator">+</span>i)
    /// &#32;    <span class="operator">*</span>i <span class="operator">+</span><span class="operator">=</span> <span class="number">2</span>;
    /// </pre>
    /// <p>Most <a href="http://doc.qt.io/qt-5/qlist.html">QList</a> functions accept an integer index rather than an iterator. For that reason, iterators are rarely useful in connection with <a href="http://doc.qt.io/qt-5/qlist.html">QList</a>. One place where STL-style iterators do make sense is as arguments to <a href="http://doc.qt.io/qt-5/qtalgorithms.html">generic algorithms</a>.</p>
    /// <p>For example, here's how to delete all the widgets stored in a <a href="http://doc.qt.io/qt-5/qlist.html">QList</a>&lt;<a href="http://doc.qt.io/qt-5/qwidget.html">QWidget</a> *&gt;:</p>
    /// <pre class="cpp"> <span class="type"><a href="http://doc.qt.io/qt-5/qlist.html#QList">QList</a></span><span class="operator">&lt;</span><span class="type"><a href="http://doc.qt.io/qt-5/qwidget.html">QWidget</a></span> <span class="operator">*</span><span class="operator">&gt;</span> list;
    ///  <span class="operator">.</span><span class="operator">.</span><span class="operator">.</span>
    ///  <a href="http://doc.qt.io/qt-5/qtalgorithms.html#qDeleteAll">qDeleteAll</a>(list<span class="operator">.</span>begin()<span class="operator">,</span> list<span class="operator">.</span>end());
    /// </pre>
    /// <p>Multiple iterators can be used on the same list. However, be aware that any non-const function call performed on the <a href="http://doc.qt.io/qt-5/qlist.html">QList</a> will render all existing iterators undefined. If you need to keep iterators over a long period of time, we recommend that you use QLinkedList rather than <a href="http://doc.qt.io/qt-5/qlist.html">QList</a>.</p>
    /// <p><b>Warning:</b> Iterators on implicitly shared containers do not work exactly like STL-iterators. You should avoid copying a container while iterators are active on that container. For more information, read <a href="http://doc.qt.io/qt-5/containers.html#implicit-sharing-iterator-problem">Implicit sharing iterator problem</a>.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_iterator_diterator(
        this_ptr: *mut crate::q_list_of_q_web_engine_script::Iterator,
    );

    /// <p><a href="http://doc.qt.io/qt-5/qlist.html">QList</a> provides both <a href="http://doc.qt.io/qt-5/containers.html#stl-style-iterators">STL-style iterators</a> and <a href="http://doc.qt.io/qt-5/containers.html#java-style-iterators">Java-style iterators</a>. The STL-style iterators are more low-level and more cumbersome to use; on the other hand, they are slightly faster and, for developers who already know STL, have the advantage of familiarity.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist-const-iterator.html">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p><a href="http://doc.qt.io/qt-5/qlist.html">QList</a> provides both <a href="http://doc.qt.io/qt-5/containers.html#stl-style-iterators">STL-style iterators</a> and <a href="http://doc.qt.io/qt-5/containers.html#java-style-iterators">Java-style iterators</a>. The STL-style iterators are more low-level and more cumbersome to use; on the other hand, they are slightly faster and, for developers who already know STL, have the advantage of familiarity.</p>
    /// <p><a href="http://doc.qt.io/qt-5/qlist.html">QList</a>&lt;T&gt;::const_iterator allows you to iterate over a <a href="http://doc.qt.io/qt-5/qlist.html">QList</a>&lt;T&gt; (or a <a href="http://doc.qt.io/qt-5/qqueue.html">QQueue</a>&lt;T&gt;). If you want to modify the <a href="http://doc.qt.io/qt-5/qlist.html">QList</a> as you iterate over it, use <a href="http://doc.qt.io/qt-5/qlist-iterator.html">QList::iterator</a> instead. It is generally good practice to use <a href="http://doc.qt.io/qt-5/qlist-const-iterator.html">QList::const_iterator</a> on a non-const <a href="http://doc.qt.io/qt-5/qlist.html">QList</a> as well, unless you need to change the <a href="http://doc.qt.io/qt-5/qlist.html">QList</a> through the iterator. Const iterators are slightly faster, and can improve code readability.</p>
    /// <p>The default <a href="http://doc.qt.io/qt-5/qlist-const-iterator.html">QList::const_iterator</a> constructor creates an uninitialized iterator. You must initialize it using a <a href="http://doc.qt.io/qt-5/qlist.html">QList</a> function like <a href="http://doc.qt.io/qt-5/qlist.html#constBegin">QList::constBegin</a>(), <a href="http://doc.qt.io/qt-5/qlist.html#constEnd">QList::constEnd</a>(), or <a href="http://doc.qt.io/qt-5/qlist.html#insert">QList::insert</a>() before you can start iterating. Here's a typical loop that prints all the items stored in a list:</p>
    /// <pre class="cpp"> <span class="type"><a href="http://doc.qt.io/qt-5/qlist.html#QList">QList</a></span><span class="operator">&lt;</span><span class="type"><a href="http://doc.qt.io/qt-5/qstring.html">QString</a></span><span class="operator">&gt;</span> list;
    ///  list<span class="operator">.</span>append(<span class="string">"January"</span>);
    ///  list<span class="operator">.</span>append(<span class="string">"February"</span>);
    ///  <span class="operator">.</span><span class="operator">.</span><span class="operator">.</span>
    ///  list<span class="operator">.</span>append(<span class="string">"December"</span>);
    ///
    ///  <span class="type"><a href="http://doc.qt.io/qt-5/qlist.html#QList">QList</a></span><span class="operator">&lt;</span><span class="type"><a href="http://doc.qt.io/qt-5/qstring.html">QString</a></span><span class="operator">&gt;</span><span class="operator">::</span>const_iterator i;
    ///  <span class="keyword">for</span> (i <span class="operator">=</span> list<span class="operator">.</span>constBegin(); i <span class="operator">!</span><span class="operator">=</span> list<span class="operator">.</span>constEnd(); <span class="operator">+</span><span class="operator">+</span>i)
    /// &#32;    cout <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="operator">*</span>i <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="type">Qt</span><span class="operator">::</span>endl;
    /// </pre>
    /// <p>Most <a href="http://doc.qt.io/qt-5/qlist.html">QList</a> functions accept an integer index rather than an iterator. For that reason, iterators are rarely useful in connection with <a href="http://doc.qt.io/qt-5/qlist.html">QList</a>. One place where STL-style iterators do make sense is as arguments to <a href="http://doc.qt.io/qt-5/qtalgorithms.html">generic algorithms</a>.</p>
    /// <p>For example, here's how to delete all the widgets stored in a <a href="http://doc.qt.io/qt-5/qlist.html">QList</a>&lt;<a href="http://doc.qt.io/qt-5/qwidget.html">QWidget</a> *&gt;:</p>
    /// <pre class="cpp"> <span class="type"><a href="http://doc.qt.io/qt-5/qlist.html#QList">QList</a></span><span class="operator">&lt;</span><span class="type"><a href="http://doc.qt.io/qt-5/qwidget.html">QWidget</a></span> <span class="operator">*</span><span class="operator">&gt;</span> list;
    ///  <span class="operator">.</span><span class="operator">.</span><span class="operator">.</span>
    ///  <a href="http://doc.qt.io/qt-5/qtalgorithms.html#qDeleteAll">qDeleteAll</a>(list<span class="operator">.</span>constBegin()<span class="operator">,</span> list<span class="operator">.</span>constEnd());
    /// </pre>
    /// <p>Multiple iterators can be used on the same list. However, be aware that any non-const function call performed on the <a href="http://doc.qt.io/qt-5/qlist.html">QList</a> will render all existing iterators undefined. If you need to keep iterators over a long period of time, we recommend that you use QLinkedList rather than <a href="http://doc.qt.io/qt-5/qlist.html">QList</a>.</p>
    /// <p><b>Warning:</b> Iterators on implicitly shared containers do not work exactly like STL-iterators. You should avoid copying a container while iterators are active on that container. For more information, read <a href="http://doc.qt.io/qt-5/containers.html#implicit-sharing-iterator-problem">Implicit sharing iterator problem</a>.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_const_iterator_dconst_iterator(
        this_ptr: *mut crate::q_list_of_q_web_engine_history_item::ConstIterator,
    );

    /// <p><a href="http://doc.qt.io/qt-5/qlist.html">QList</a> provides both <a href="http://doc.qt.io/qt-5/containers.html#stl-style-iterators">STL-style iterators</a> and <a href="http://doc.qt.io/qt-5/containers.html#java-style-iterators">Java-style iterators</a>. The STL-style iterators are more low-level and more cumbersome to use; on the other hand, they are slightly faster and, for developers who already know STL, have the advantage of familiarity.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist-const-iterator.html">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p><a href="http://doc.qt.io/qt-5/qlist.html">QList</a> provides both <a href="http://doc.qt.io/qt-5/containers.html#stl-style-iterators">STL-style iterators</a> and <a href="http://doc.qt.io/qt-5/containers.html#java-style-iterators">Java-style iterators</a>. The STL-style iterators are more low-level and more cumbersome to use; on the other hand, they are slightly faster and, for developers who already know STL, have the advantage of familiarity.</p>
    /// <p><a href="http://doc.qt.io/qt-5/qlist.html">QList</a>&lt;T&gt;::const_iterator allows you to iterate over a <a href="http://doc.qt.io/qt-5/qlist.html">QList</a>&lt;T&gt; (or a <a href="http://doc.qt.io/qt-5/qqueue.html">QQueue</a>&lt;T&gt;). If you want to modify the <a href="http://doc.qt.io/qt-5/qlist.html">QList</a> as you iterate over it, use <a href="http://doc.qt.io/qt-5/qlist-iterator.html">QList::iterator</a> instead. It is generally good practice to use <a href="http://doc.qt.io/qt-5/qlist-const-iterator.html">QList::const_iterator</a> on a non-const <a href="http://doc.qt.io/qt-5/qlist.html">QList</a> as well, unless you need to change the <a href="http://doc.qt.io/qt-5/qlist.html">QList</a> through the iterator. Const iterators are slightly faster, and can improve code readability.</p>
    /// <p>The default <a href="http://doc.qt.io/qt-5/qlist-const-iterator.html">QList::const_iterator</a> constructor creates an uninitialized iterator. You must initialize it using a <a href="http://doc.qt.io/qt-5/qlist.html">QList</a> function like <a href="http://doc.qt.io/qt-5/qlist.html#constBegin">QList::constBegin</a>(), <a href="http://doc.qt.io/qt-5/qlist.html#constEnd">QList::constEnd</a>(), or <a href="http://doc.qt.io/qt-5/qlist.html#insert">QList::insert</a>() before you can start iterating. Here's a typical loop that prints all the items stored in a list:</p>
    /// <pre class="cpp"> <span class="type"><a href="http://doc.qt.io/qt-5/qlist.html#QList">QList</a></span><span class="operator">&lt;</span><span class="type"><a href="http://doc.qt.io/qt-5/qstring.html">QString</a></span><span class="operator">&gt;</span> list;
    ///  list<span class="operator">.</span>append(<span class="string">"January"</span>);
    ///  list<span class="operator">.</span>append(<span class="string">"February"</span>);
    ///  <span class="operator">.</span><span class="operator">.</span><span class="operator">.</span>
    ///  list<span class="operator">.</span>append(<span class="string">"December"</span>);
    ///
    ///  <span class="type"><a href="http://doc.qt.io/qt-5/qlist.html#QList">QList</a></span><span class="operator">&lt;</span><span class="type"><a href="http://doc.qt.io/qt-5/qstring.html">QString</a></span><span class="operator">&gt;</span><span class="operator">::</span>const_iterator i;
    ///  <span class="keyword">for</span> (i <span class="operator">=</span> list<span class="operator">.</span>constBegin(); i <span class="operator">!</span><span class="operator">=</span> list<span class="operator">.</span>constEnd(); <span class="operator">+</span><span class="operator">+</span>i)
    /// &#32;    cout <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="operator">*</span>i <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="type">Qt</span><span class="operator">::</span>endl;
    /// </pre>
    /// <p>Most <a href="http://doc.qt.io/qt-5/qlist.html">QList</a> functions accept an integer index rather than an iterator. For that reason, iterators are rarely useful in connection with <a href="http://doc.qt.io/qt-5/qlist.html">QList</a>. One place where STL-style iterators do make sense is as arguments to <a href="http://doc.qt.io/qt-5/qtalgorithms.html">generic algorithms</a>.</p>
    /// <p>For example, here's how to delete all the widgets stored in a <a href="http://doc.qt.io/qt-5/qlist.html">QList</a>&lt;<a href="http://doc.qt.io/qt-5/qwidget.html">QWidget</a> *&gt;:</p>
    /// <pre class="cpp"> <span class="type"><a href="http://doc.qt.io/qt-5/qlist.html#QList">QList</a></span><span class="operator">&lt;</span><span class="type"><a href="http://doc.qt.io/qt-5/qwidget.html">QWidget</a></span> <span class="operator">*</span><span class="operator">&gt;</span> list;
    ///  <span class="operator">.</span><span class="operator">.</span><span class="operator">.</span>
    ///  <a href="http://doc.qt.io/qt-5/qtalgorithms.html#qDeleteAll">qDeleteAll</a>(list<span class="operator">.</span>constBegin()<span class="operator">,</span> list<span class="operator">.</span>constEnd());
    /// </pre>
    /// <p>Multiple iterators can be used on the same list. However, be aware that any non-const function call performed on the <a href="http://doc.qt.io/qt-5/qlist.html">QList</a> will render all existing iterators undefined. If you need to keep iterators over a long period of time, we recommend that you use QLinkedList rather than <a href="http://doc.qt.io/qt-5/qlist.html">QList</a>.</p>
    /// <p><b>Warning:</b> Iterators on implicitly shared containers do not work exactly like STL-iterators. You should avoid copying a container while iterators are active on that container. For more information, read <a href="http://doc.qt.io/qt-5/containers.html#implicit-sharing-iterator-problem">Implicit sharing iterator problem</a>.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_const_iterator_dconst_iterator(
        this_ptr: *mut crate::q_list_of_q_web_engine_script::ConstIterator,
    );

    /// <p>A guarded pointer, QPointer&lt;T&gt;, behaves like a normal C++ pointer <code>T *</code>, except that it is automatically cleared when the referenced object is destroyed (unlike normal C++ pointers, which become "dangling pointers" in such cases). <code>T</code> must be a subclass of <a href="http://doc.qt.io/qt-5/qobject.html">QObject</a>.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qpointer.html">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>A guarded pointer, QPointer&lt;T&gt;, behaves like a normal C++ pointer <code>T *</code>, except that it is automatically cleared when the referenced object is destroyed (unlike normal C++ pointers, which become "dangling pointers" in such cases). <code>T</code> must be a subclass of <a href="http://doc.qt.io/qt-5/qobject.html">QObject</a>.</p>
    /// <p>Guarded pointers are useful whenever you need to store a pointer to a <a href="http://doc.qt.io/qt-5/qobject.html">QObject</a> that is owned by someone else, and therefore might be destroyed while you still hold a reference to it. You can safely test the pointer for validity.</p>
    /// <p>Note that Qt 5 introduces a slight change in behavior when using QPointer.</p>
    /// <ul>
    /// <li>When using QPointer on a <a href="http://doc.qt.io/qt-5/qwidget.html">QWidget</a> (or a subclass of <a href="http://doc.qt.io/qt-5/qwidget.html">QWidget</a>), previously the QPointer would be cleared by the <a href="http://doc.qt.io/qt-5/qwidget.html">QWidget</a> destructor. Now, the QPointer is cleared by the <a href="http://doc.qt.io/qt-5/qobject.html">QObject</a> destructor (since this is when <a href="http://doc.qt.io/qt-5/qweakpointer.html">QWeakPointer</a> objects are cleared). Any QPointers tracking a widget will <b>NOT</b> be cleared before the <a href="http://doc.qt.io/qt-5/qwidget.html">QWidget</a> destructor destroys the children for the widget being tracked.</li>
    /// </ul>
    /// <p>Qt also provides <a href="http://doc.qt.io/qt-5/qsharedpointer.html">QSharedPointer</a>, an implementation of a reference-counted shared pointer object, which can be used to maintain a collection of references to an individual pointer.</p>
    /// <p>Example:</p>
    /// <pre class="cpp">     <span class="type"><a href="http://doc.qt.io/qt-5/qpointer.html#QPointer">QPointer</a></span><span class="operator">&lt;</span><span class="type"><a href="http://doc.qt.io/qt-5/qlabel.html">QLabel</a></span><span class="operator">&gt;</span> label <span class="operator">=</span> <span class="keyword">new</span> <span class="type"><a href="http://doc.qt.io/qt-5/qlabel.html">QLabel</a></span>;
    /// &#32;    label<span class="operator">-</span><span class="operator">&gt;</span>setText(<span class="string">"&amp;Status:"</span>);
    /// &#32;    ...
    /// &#32;    <span class="keyword">if</span> (label)
    /// &#32;   &#32;    label<span class="operator">-</span><span class="operator">&gt;</span>show();
    /// </pre>
    /// <p>If the <a href="http://doc.qt.io/qt-5/qlabel.html">QLabel</a> is deleted in the meantime, the <code>label</code> variable will hold <code>nullptr</code> instead of an invalid address, and the last line will never be executed.</p>
    /// <p>The functions and operators available with a QPointer are the same as those available with a normal unguarded pointer, except the pointer arithmetic operators (<code>+</code>, <code>-</code>, <code>++</code>, and <code>--</code>), which are normally used only with arrays of objects.</p>
    /// <p>Use QPointers like normal pointers and you will not need to read this class documentation.</p>
    /// <p>For creating guarded pointers, you can construct or assign to them from a T* or from another guarded pointer of the same type. You can compare them with each other using operator==() and operator!=(), or test for <code>nullptr</code> with <a href="http://doc.qt.io/qt-5/qpointer.html#isNull">isNull</a>(). You can dereference them using either the <code>*x</code> or the <code>x-&gt;member</code> notation.</p>
    /// <p>A guarded pointer will automatically cast to a <code>T</code> *, so you can freely mix guarded and unguarded pointers. This means that if you have a QPointer&lt;<a href="http://doc.qt.io/qt-5/qwidget.html">QWidget</a>&gt;, you can pass it to a function that requires a <a href="http://doc.qt.io/qt-5/qwidget.html">QWidget</a> *. For this reason, it is of little value to declare functions to take a QPointer as a parameter; just use normal pointers. Use a QPointer when you are storing a pointer over time.</p>
    /// <p>Note that class <code>T</code> must inherit <a href="http://doc.qt.io/qt-5/qobject.html">QObject</a>, or a compilation or link error will result.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QPointer_QWebEnginePage_dQPointer(
        this_ptr: *mut crate::QPointerOfQWebEnginePage,
    );

    /// <p>A guarded pointer, QPointer&lt;T&gt;, behaves like a normal C++ pointer <code>T *</code>, except that it is automatically cleared when the referenced object is destroyed (unlike normal C++ pointers, which become "dangling pointers" in such cases). <code>T</code> must be a subclass of <a href="http://doc.qt.io/qt-5/qobject.html">QObject</a>.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qpointer.html">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>A guarded pointer, QPointer&lt;T&gt;, behaves like a normal C++ pointer <code>T *</code>, except that it is automatically cleared when the referenced object is destroyed (unlike normal C++ pointers, which become "dangling pointers" in such cases). <code>T</code> must be a subclass of <a href="http://doc.qt.io/qt-5/qobject.html">QObject</a>.</p>
    /// <p>Guarded pointers are useful whenever you need to store a pointer to a <a href="http://doc.qt.io/qt-5/qobject.html">QObject</a> that is owned by someone else, and therefore might be destroyed while you still hold a reference to it. You can safely test the pointer for validity.</p>
    /// <p>Note that Qt 5 introduces a slight change in behavior when using QPointer.</p>
    /// <ul>
    /// <li>When using QPointer on a <a href="http://doc.qt.io/qt-5/qwidget.html">QWidget</a> (or a subclass of <a href="http://doc.qt.io/qt-5/qwidget.html">QWidget</a>), previously the QPointer would be cleared by the <a href="http://doc.qt.io/qt-5/qwidget.html">QWidget</a> destructor. Now, the QPointer is cleared by the <a href="http://doc.qt.io/qt-5/qobject.html">QObject</a> destructor (since this is when <a href="http://doc.qt.io/qt-5/qweakpointer.html">QWeakPointer</a> objects are cleared). Any QPointers tracking a widget will <b>NOT</b> be cleared before the <a href="http://doc.qt.io/qt-5/qwidget.html">QWidget</a> destructor destroys the children for the widget being tracked.</li>
    /// </ul>
    /// <p>Qt also provides <a href="http://doc.qt.io/qt-5/qsharedpointer.html">QSharedPointer</a>, an implementation of a reference-counted shared pointer object, which can be used to maintain a collection of references to an individual pointer.</p>
    /// <p>Example:</p>
    /// <pre class="cpp">     <span class="type"><a href="http://doc.qt.io/qt-5/qpointer.html#QPointer">QPointer</a></span><span class="operator">&lt;</span><span class="type"><a href="http://doc.qt.io/qt-5/qlabel.html">QLabel</a></span><span class="operator">&gt;</span> label <span class="operator">=</span> <span class="keyword">new</span> <span class="type"><a href="http://doc.qt.io/qt-5/qlabel.html">QLabel</a></span>;
    /// &#32;    label<span class="operator">-</span><span class="operator">&gt;</span>setText(<span class="string">"&amp;Status:"</span>);
    /// &#32;    ...
    /// &#32;    <span class="keyword">if</span> (label)
    /// &#32;   &#32;    label<span class="operator">-</span><span class="operator">&gt;</span>show();
    /// </pre>
    /// <p>If the <a href="http://doc.qt.io/qt-5/qlabel.html">QLabel</a> is deleted in the meantime, the <code>label</code> variable will hold <code>nullptr</code> instead of an invalid address, and the last line will never be executed.</p>
    /// <p>The functions and operators available with a QPointer are the same as those available with a normal unguarded pointer, except the pointer arithmetic operators (<code>+</code>, <code>-</code>, <code>++</code>, and <code>--</code>), which are normally used only with arrays of objects.</p>
    /// <p>Use QPointers like normal pointers and you will not need to read this class documentation.</p>
    /// <p>For creating guarded pointers, you can construct or assign to them from a T* or from another guarded pointer of the same type. You can compare them with each other using operator==() and operator!=(), or test for <code>nullptr</code> with <a href="http://doc.qt.io/qt-5/qpointer.html#isNull">isNull</a>(). You can dereference them using either the <code>*x</code> or the <code>x-&gt;member</code> notation.</p>
    /// <p>A guarded pointer will automatically cast to a <code>T</code> *, so you can freely mix guarded and unguarded pointers. This means that if you have a QPointer&lt;<a href="http://doc.qt.io/qt-5/qwidget.html">QWidget</a>&gt;, you can pass it to a function that requires a <a href="http://doc.qt.io/qt-5/qwidget.html">QWidget</a> *. For this reason, it is of little value to declare functions to take a QPointer as a parameter; just use normal pointers. Use a QPointer when you are storing a pointer over time.</p>
    /// <p>Note that class <code>T</code> must inherit <a href="http://doc.qt.io/qt-5/qobject.html">QObject</a>, or a compilation or link error will result.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QPointer_QWebEnginePage_QPointer2(
        other: *const crate::QPointerOfQWebEnginePage,
    ) -> *mut crate::QPointerOfQWebEnginePage;

    /// <p>A guarded pointer, QPointer&lt;T&gt;, behaves like a normal C++ pointer <code>T *</code>, except that it is automatically cleared when the referenced object is destroyed (unlike normal C++ pointers, which become "dangling pointers" in such cases). <code>T</code> must be a subclass of <a href="http://doc.qt.io/qt-5/qobject.html">QObject</a>.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qpointer.html">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>A guarded pointer, QPointer&lt;T&gt;, behaves like a normal C++ pointer <code>T *</code>, except that it is automatically cleared when the referenced object is destroyed (unlike normal C++ pointers, which become "dangling pointers" in such cases). <code>T</code> must be a subclass of <a href="http://doc.qt.io/qt-5/qobject.html">QObject</a>.</p>
    /// <p>Guarded pointers are useful whenever you need to store a pointer to a <a href="http://doc.qt.io/qt-5/qobject.html">QObject</a> that is owned by someone else, and therefore might be destroyed while you still hold a reference to it. You can safely test the pointer for validity.</p>
    /// <p>Note that Qt 5 introduces a slight change in behavior when using QPointer.</p>
    /// <ul>
    /// <li>When using QPointer on a <a href="http://doc.qt.io/qt-5/qwidget.html">QWidget</a> (or a subclass of <a href="http://doc.qt.io/qt-5/qwidget.html">QWidget</a>), previously the QPointer would be cleared by the <a href="http://doc.qt.io/qt-5/qwidget.html">QWidget</a> destructor. Now, the QPointer is cleared by the <a href="http://doc.qt.io/qt-5/qobject.html">QObject</a> destructor (since this is when <a href="http://doc.qt.io/qt-5/qweakpointer.html">QWeakPointer</a> objects are cleared). Any QPointers tracking a widget will <b>NOT</b> be cleared before the <a href="http://doc.qt.io/qt-5/qwidget.html">QWidget</a> destructor destroys the children for the widget being tracked.</li>
    /// </ul>
    /// <p>Qt also provides <a href="http://doc.qt.io/qt-5/qsharedpointer.html">QSharedPointer</a>, an implementation of a reference-counted shared pointer object, which can be used to maintain a collection of references to an individual pointer.</p>
    /// <p>Example:</p>
    /// <pre class="cpp">     <span class="type"><a href="http://doc.qt.io/qt-5/qpointer.html#QPointer">QPointer</a></span><span class="operator">&lt;</span><span class="type"><a href="http://doc.qt.io/qt-5/qlabel.html">QLabel</a></span><span class="operator">&gt;</span> label <span class="operator">=</span> <span class="keyword">new</span> <span class="type"><a href="http://doc.qt.io/qt-5/qlabel.html">QLabel</a></span>;
    /// &#32;    label<span class="operator">-</span><span class="operator">&gt;</span>setText(<span class="string">"&amp;Status:"</span>);
    /// &#32;    ...
    /// &#32;    <span class="keyword">if</span> (label)
    /// &#32;   &#32;    label<span class="operator">-</span><span class="operator">&gt;</span>show();
    /// </pre>
    /// <p>If the <a href="http://doc.qt.io/qt-5/qlabel.html">QLabel</a> is deleted in the meantime, the <code>label</code> variable will hold <code>nullptr</code> instead of an invalid address, and the last line will never be executed.</p>
    /// <p>The functions and operators available with a QPointer are the same as those available with a normal unguarded pointer, except the pointer arithmetic operators (<code>+</code>, <code>-</code>, <code>++</code>, and <code>--</code>), which are normally used only with arrays of objects.</p>
    /// <p>Use QPointers like normal pointers and you will not need to read this class documentation.</p>
    /// <p>For creating guarded pointers, you can construct or assign to them from a T* or from another guarded pointer of the same type. You can compare them with each other using operator==() and operator!=(), or test for <code>nullptr</code> with <a href="http://doc.qt.io/qt-5/qpointer.html#isNull">isNull</a>(). You can dereference them using either the <code>*x</code> or the <code>x-&gt;member</code> notation.</p>
    /// <p>A guarded pointer will automatically cast to a <code>T</code> *, so you can freely mix guarded and unguarded pointers. This means that if you have a QPointer&lt;<a href="http://doc.qt.io/qt-5/qwidget.html">QWidget</a>&gt;, you can pass it to a function that requires a <a href="http://doc.qt.io/qt-5/qwidget.html">QWidget</a> *. For this reason, it is of little value to declare functions to take a QPointer as a parameter; just use normal pointers. Use a QPointer when you are storing a pointer over time.</p>
    /// <p>Note that class <code>T</code> must inherit <a href="http://doc.qt.io/qt-5/qobject.html">QObject</a>, or a compilation or link error will result.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QPointer_QWebEnginePage_operator_2(
        this_ptr: *mut crate::QPointerOfQWebEnginePage,
        other: *const crate::QPointerOfQWebEnginePage,
    ) -> *mut crate::QPointerOfQWebEnginePage;

    /// <p>Returns the index position of the first occurrence of <i>value</i> in the list, searching forward from index position <i>from</i>. Returns -1 if no item matched.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#indexOf">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns the index position of the first occurrence of <i>value</i> in the list, searching forward from index position <i>from</i>. Returns -1 if no item matched.</p>
    /// <p>Example:</p>
    /// <pre class="cpp"> <span class="type"><a href="http://doc.qt.io/qt-5/qlist.html#QList">QList</a></span><span class="operator">&lt;</span><span class="type"><a href="http://doc.qt.io/qt-5/qstring.html">QString</a></span><span class="operator">&gt;</span> list;
    ///  list <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"A"</span> <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"B"</span> <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"C"</span> <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"B"</span> <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"A"</span>;
    ///  list<span class="operator">.</span>indexOf(<span class="string">"B"</span>);          <span class="comment">// returns 1</span>
    ///  list<span class="operator">.</span>indexOf(<span class="string">"B"</span><span class="operator">,</span> <span class="number">1</span>);       <span class="comment">// returns 1</span>
    ///  list<span class="operator">.</span>indexOf(<span class="string">"B"</span><span class="operator">,</span> <span class="number">2</span>);       <span class="comment">// returns 3</span>
    ///  list<span class="operator">.</span>indexOf(<span class="string">"X"</span>);          <span class="comment">// returns -1</span>
    /// </pre>
    /// <p>This function requires the value type to have an implementation of <code>operator==()</code>.</p>
    /// <p>Note that <a href="http://doc.qt.io/qt-5/qlist.html">QList</a> uses 0-based indexes, just like C++ arrays. Negative indexes are not supported with the exception of the value mentioned above.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#lastIndexOf">lastIndexOf</a>() and <a href="http://doc.qt.io/qt-5/qlist.html#contains">contains</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_indexOf1(
        this_ptr: *const crate::QListOfQWebEngineScript,
        t: *const crate::QWebEngineScript,
    ) -> ::std::os::raw::c_int;

    /// <p>Returns the index position of the last occurrence of <i>value</i> in the list, searching backward from index position <i>from</i>. If <i>from</i> is -1 (the default), the search starts at the last item. Returns -1 if no item matched.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#lastIndexOf">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns the index position of the last occurrence of <i>value</i> in the list, searching backward from index position <i>from</i>. If <i>from</i> is -1 (the default), the search starts at the last item. Returns -1 if no item matched.</p>
    /// <p>Example:</p>
    /// <pre class="cpp"> <span class="type"><a href="http://doc.qt.io/qt-5/qlist.html#QList">QList</a></span><span class="operator">&lt;</span><span class="type"><a href="http://doc.qt.io/qt-5/qstring.html">QString</a></span><span class="operator">&gt;</span> list;
    ///  list <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"A"</span> <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"B"</span> <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"C"</span> <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"B"</span> <span class="operator">&lt;</span><span class="operator">&lt;</span> <span class="string">"A"</span>;
    ///  list<span class="operator">.</span>lastIndexOf(<span class="string">"B"</span>);      <span class="comment">// returns 3</span>
    ///  list<span class="operator">.</span>lastIndexOf(<span class="string">"B"</span><span class="operator">,</span> <span class="number">3</span>);   <span class="comment">// returns 3</span>
    ///  list<span class="operator">.</span>lastIndexOf(<span class="string">"B"</span><span class="operator">,</span> <span class="number">2</span>);   <span class="comment">// returns 1</span>
    ///  list<span class="operator">.</span>lastIndexOf(<span class="string">"X"</span>);      <span class="comment">// returns -1</span>
    /// </pre>
    /// <p>This function requires the value type to have an implementation of <code>operator==()</code>.</p>
    /// <p>Note that <a href="http://doc.qt.io/qt-5/qlist.html">QList</a> uses 0-based indexes, just like C++ arrays. Negative indexes are not supported with the exception of the value mentioned above.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qlist.html#indexOf">indexOf</a>().</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_lastIndexOf1(
        this_ptr: *const crate::QListOfQWebEngineScript,
        t: *const crate::QWebEngineScript,
    ) -> ::std::os::raw::c_int;

    /// <p>Returns a sub-list which includes elements from this list, starting at position <i>pos</i>. If <i>length</i> is -1 (the default), all elements from <i>pos</i> are included; otherwise <i>length</i> elements (or all remaining elements if there are less than <i>length</i> elements) are included.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#mid">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns a sub-list which includes elements from this list, starting at position <i>pos</i>. If <i>length</i> is -1 (the default), all elements from <i>pos</i> are included; otherwise <i>length</i> elements (or all remaining elements if there are less than <i>length</i> elements) are included.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineHistoryItem_mid1(
        this_ptr: *const crate::QListOfQWebEngineHistoryItem,
        pos: ::std::os::raw::c_int,
    ) -> *mut crate::QListOfQWebEngineHistoryItem;

    /// <p>Returns a sub-list which includes elements from this list, starting at position <i>pos</i>. If <i>length</i> is -1 (the default), all elements from <i>pos</i> are included; otherwise <i>length</i> elements (or all remaining elements if there are less than <i>length</i> elements) are included.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qlist.html#mid">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns a sub-list which includes elements from this list, starting at position <i>pos</i>. If <i>length</i> is -1 (the default), all elements from <i>pos</i> are included; otherwise <i>length</i> elements (or all remaining elements if there are less than <i>length</i> elements) are included.</p></div>
    pub fn ctr_qt_web_engine_widgets_ffi_QList_QWebEngineScript_mid1(
        this_ptr: *const crate::QListOfQWebEngineScript,
        pos: ::std::os::raw::c_int,
    ) -> *mut crate::QListOfQWebEngineScript;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_QWebEnginePage3(
        profile: *mut crate::QWebEngineProfile,
    ) -> *mut crate::QWebEnginePage;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_triggerAction1(
        this_ptr: *mut crate::QWebEnginePage,
        action: crate::q_web_engine_page::WebAction,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_findText1(
        this_ptr: *mut crate::QWebEnginePage,
        subString: *const ::qt_core::QString,
        options: ::std::os::raw::c_int,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_findText2(
        this_ptr: *mut crate::QWebEnginePage,
        subString: *const ::qt_core::QString,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_download1(
        this_ptr: *mut crate::QWebEnginePage,
        url: *const ::qt_core::QUrl,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_setHtml1(
        this_ptr: *mut crate::QWebEnginePage,
        html: *const ::qt_core::QString,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_setContent1(
        this_ptr: *mut crate::QWebEnginePage,
        data: *const ::qt_core::QByteArray,
        mimeType: *const ::qt_core::QString,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_setContent2(
        this_ptr: *mut crate::QWebEnginePage,
        data: *const ::qt_core::QByteArray,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_save1(
        this_ptr: *const crate::QWebEnginePage,
        filePath: *const ::qt_core::QString,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEnginePage_printToPdf2(
        this_ptr: *mut crate::QWebEnginePage,
        filePath: *const ::qt_core::QString,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineProfile_QWebEngineProfile3(
        name: *const ::qt_core::QString,
    ) -> *mut crate::QWebEngineProfile;

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineProfile_setUseForGlobalCertificateVerification1(
        this_ptr: *mut crate::QWebEngineProfile,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineView_setHtml1(
        this_ptr: *mut crate::QWebEngineView,
        html: *const ::qt_core::QString,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineView_setContent1(
        this_ptr: *mut crate::QWebEngineView,
        data: *const ::qt_core::QByteArray,
        mimeType: *const ::qt_core::QString,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineView_setContent2(
        this_ptr: *mut crate::QWebEngineView,
        data: *const ::qt_core::QByteArray,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineView_triggerPageAction1(
        this_ptr: *mut crate::QWebEngineView,
        action: crate::q_web_engine_page::WebAction,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineView_findText1(
        this_ptr: *mut crate::QWebEngineView,
        subString: *const ::qt_core::QString,
        options: ::std::os::raw::c_int,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_QWebEngineView_findText2(
        this_ptr: *mut crate::QWebEngineView,
        subString: *const ::qt_core::QString,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_QWebEngineDownloadItem_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::QWebEngineDownloadItem;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_QObject_ptr(
        ptr: *mut crate::QWebEngineDownloadItem,
    ) -> *mut ::qt_core::QObject;

    pub fn ctr_qt_web_engine_widgets_ffi_dynamic_cast_QWebEngineDownloadItem_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::QWebEngineDownloadItem;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_QWebEnginePage_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::QWebEnginePage;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_QObject_ptr1(
        ptr: *mut crate::QWebEnginePage,
    ) -> *mut ::qt_core::QObject;

    pub fn ctr_qt_web_engine_widgets_ffi_dynamic_cast_QWebEnginePage_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::QWebEnginePage;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_QWebEngineProfile_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::QWebEngineProfile;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_QObject_ptr2(
        ptr: *mut crate::QWebEngineProfile,
    ) -> *mut ::qt_core::QObject;

    pub fn ctr_qt_web_engine_widgets_ffi_dynamic_cast_QWebEngineProfile_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::QWebEngineProfile;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_QWebEngineView_ptr(
        ptr: *mut ::qt_widgets::QWidget,
    ) -> *mut crate::QWebEngineView;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_QWidget_ptr(
        ptr: *mut crate::QWebEngineView,
    ) -> *mut ::qt_widgets::QWidget;

    pub fn ctr_qt_web_engine_widgets_ffi_dynamic_cast_QWebEngineView_ptr(
        ptr: *mut ::qt_widgets::QWidget,
    ) -> *mut crate::QWebEngineView;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_QWebEngineView_ptr1(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::QWebEngineView;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_QObject_ptr3(
        ptr: *mut crate::QWebEngineView,
    ) -> *mut ::qt_core::QObject;

    pub fn ctr_qt_web_engine_widgets_ffi_dynamic_cast_QWebEngineView_ptr1(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::QWebEngineView;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_QWebEngineView_ptr2(
        ptr: *mut ::qt_gui::QPaintDevice,
    ) -> *mut crate::QWebEngineView;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_QPaintDevice_ptr(
        ptr: *mut crate::QWebEngineView,
    ) -> *mut ::qt_gui::QPaintDevice;

    pub fn ctr_qt_web_engine_widgets_ffi_dynamic_cast_QWebEngineView_ptr2(
        ptr: *mut ::qt_gui::QPaintDevice,
    ) -> *mut crate::QWebEngineView;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineFullScreenRequest_staticMetaObject(
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineFullScreenRequest_metaObject(
        this_ptr: *const crate::SlotOfQWebEngineFullScreenRequest,
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineFullScreenRequest_qt_metacast(
        this_ptr: *mut crate::SlotOfQWebEngineFullScreenRequest,
        arg1: *const ::std::os::raw::c_char,
    ) -> *mut ::std::ffi::c_void;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineFullScreenRequest_qt_metacall(
        this_ptr: *mut crate::SlotOfQWebEngineFullScreenRequest,
        arg1: ::qt_core::q_meta_object::Call,
        arg2: ::std::os::raw::c_int,
        arg3: *mut *mut ::std::ffi::c_void,
    ) -> ::std::os::raw::c_int;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineFullScreenRequest_tr(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineFullScreenRequest_trUtf8(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineFullScreenRequest_slot_(
        this_ptr: *mut crate::SlotOfQWebEngineFullScreenRequest,
        arg0: *const crate::QWebEngineFullScreenRequest,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineFullScreenRequest_staticMetaObject(
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineFullScreenRequest_metaObject(
        this_ptr: *const crate::SignalOfQWebEngineFullScreenRequest,
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineFullScreenRequest_qt_metacast(
        this_ptr: *mut crate::SignalOfQWebEngineFullScreenRequest,
        arg1: *const ::std::os::raw::c_char,
    ) -> *mut ::std::ffi::c_void;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineFullScreenRequest_qt_metacall(
        this_ptr: *mut crate::SignalOfQWebEngineFullScreenRequest,
        arg1: ::qt_core::q_meta_object::Call,
        arg2: ::std::os::raw::c_int,
        arg3: *mut *mut ::std::ffi::c_void,
    ) -> ::std::os::raw::c_int;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineFullScreenRequest_tr(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineFullScreenRequest_trUtf8(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineFullScreenRequest_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineFullScreenRequest(
    ) -> *mut crate::SignalOfQWebEngineFullScreenRequest;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineFullScreenRequest_emit_(
        this_ptr: *mut crate::SignalOfQWebEngineFullScreenRequest,
        arg0: *const crate::QWebEngineFullScreenRequest,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QString_const_ref_bool_staticMetaObject(
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QString_const_ref_bool_metaObject(
        this_ptr: *const crate::SlotOfQStringBool,
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QString_const_ref_bool_qt_metacast(
        this_ptr: *mut crate::SlotOfQStringBool,
        arg1: *const ::std::os::raw::c_char,
    ) -> *mut ::std::ffi::c_void;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QString_const_ref_bool_qt_metacall(
        this_ptr: *mut crate::SlotOfQStringBool,
        arg1: ::qt_core::q_meta_object::Call,
        arg2: ::std::os::raw::c_int,
        arg3: *mut *mut ::std::ffi::c_void,
    ) -> ::std::os::raw::c_int;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QString_const_ref_bool_tr(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QString_const_ref_bool_trUtf8(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QString_const_ref_bool_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QString_const_ref_bool(
        parent: *mut ::qt_core::QObject,
        callback: ::std::option::Option<
            extern "C" fn(*mut ::std::ffi::c_void, *const ::qt_core::QString, bool),
        >,
        deleter: ::std::option::Option<extern "C" fn(*mut ::std::ffi::c_void)>,
        data: *mut ::std::ffi::c_void,
    ) -> *mut crate::SlotOfQStringBool;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QString_const_ref_bool_set(
        this_ptr: *mut crate::SlotOfQStringBool,
        callback: ::std::option::Option<
            extern "C" fn(*mut ::std::ffi::c_void, *const ::qt_core::QString, bool),
        >,
        deleter: ::std::option::Option<extern "C" fn(*mut ::std::ffi::c_void)>,
        data: *mut ::std::ffi::c_void,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QString_const_ref_bool_slot_(
        this_ptr: *mut crate::SlotOfQStringBool,
        arg0: *const ::qt_core::QString,
        arg1: bool,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QString_const_ref_bool_staticMetaObject(
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QString_const_ref_bool_metaObject(
        this_ptr: *const crate::SignalOfQStringBool,
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QString_const_ref_bool_qt_metacast(
        this_ptr: *mut crate::SignalOfQStringBool,
        arg1: *const ::std::os::raw::c_char,
    ) -> *mut ::std::ffi::c_void;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QString_const_ref_bool_qt_metacall(
        this_ptr: *mut crate::SignalOfQStringBool,
        arg1: ::qt_core::q_meta_object::Call,
        arg2: ::std::os::raw::c_int,
        arg3: *mut *mut ::std::ffi::c_void,
    ) -> ::std::os::raw::c_int;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QString_const_ref_bool_tr(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QString_const_ref_bool_trUtf8(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QString_const_ref_bool_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QString_const_ref_bool(
    ) -> *mut crate::SignalOfQStringBool;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QString_const_ref_bool_emit_(
        this_ptr: *mut crate::SignalOfQStringBool,
        arg0: *const ::qt_core::QString,
        arg1: bool,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineDownloadItem_ptr_staticMetaObject(
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineDownloadItem_ptr_metaObject(
        this_ptr: *const crate::SlotOfQWebEngineDownloadItem,
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineDownloadItem_ptr_qt_metacast(
        this_ptr: *mut crate::SlotOfQWebEngineDownloadItem,
        arg1: *const ::std::os::raw::c_char,
    ) -> *mut ::std::ffi::c_void;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineDownloadItem_ptr_qt_metacall(
        this_ptr: *mut crate::SlotOfQWebEngineDownloadItem,
        arg1: ::qt_core::q_meta_object::Call,
        arg2: ::std::os::raw::c_int,
        arg3: *mut *mut ::std::ffi::c_void,
    ) -> ::std::os::raw::c_int;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineDownloadItem_ptr_tr(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineDownloadItem_ptr_trUtf8(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineDownloadItem_ptr_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineDownloadItem_ptr(
        parent: *mut ::qt_core::QObject,
        callback: ::std::option::Option<
            extern "C" fn(*mut ::std::ffi::c_void, *mut crate::QWebEngineDownloadItem),
        >,
        deleter: ::std::option::Option<extern "C" fn(*mut ::std::ffi::c_void)>,
        data: *mut ::std::ffi::c_void,
    ) -> *mut crate::SlotOfQWebEngineDownloadItem;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineDownloadItem_ptr_set(
        this_ptr: *mut crate::SlotOfQWebEngineDownloadItem,
        callback: ::std::option::Option<
            extern "C" fn(*mut ::std::ffi::c_void, *mut crate::QWebEngineDownloadItem),
        >,
        deleter: ::std::option::Option<extern "C" fn(*mut ::std::ffi::c_void)>,
        data: *mut ::std::ffi::c_void,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineDownloadItem_ptr_slot_(
        this_ptr: *mut crate::SlotOfQWebEngineDownloadItem,
        arg0: *mut crate::QWebEngineDownloadItem,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineDownloadItem_ptr_staticMetaObject(
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineDownloadItem_ptr_metaObject(
        this_ptr: *const crate::SignalOfQWebEngineDownloadItem,
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineDownloadItem_ptr_qt_metacast(
        this_ptr: *mut crate::SignalOfQWebEngineDownloadItem,
        arg1: *const ::std::os::raw::c_char,
    ) -> *mut ::std::ffi::c_void;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineDownloadItem_ptr_qt_metacall(
        this_ptr: *mut crate::SignalOfQWebEngineDownloadItem,
        arg1: ::qt_core::q_meta_object::Call,
        arg2: ::std::os::raw::c_int,
        arg3: *mut *mut ::std::ffi::c_void,
    ) -> ::std::os::raw::c_int;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineDownloadItem_ptr_tr(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineDownloadItem_ptr_trUtf8(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineDownloadItem_ptr_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineDownloadItem_ptr(
    ) -> *mut crate::SignalOfQWebEngineDownloadItem;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineDownloadItem_ptr_emit_(
        this_ptr: *mut crate::SignalOfQWebEngineDownloadItem,
        arg0: *mut crate::QWebEngineDownloadItem,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineClientCertificateSelection_staticMetaObject(
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineClientCertificateSelection_metaObject(
        this_ptr: *const crate::SlotOfQWebEngineClientCertificateSelection,
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineClientCertificateSelection_qt_metacast(
        this_ptr: *mut crate::SlotOfQWebEngineClientCertificateSelection,
        arg1: *const ::std::os::raw::c_char,
    ) -> *mut ::std::ffi::c_void;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineClientCertificateSelection_qt_metacall(
        this_ptr: *mut crate::SlotOfQWebEngineClientCertificateSelection,
        arg1: ::qt_core::q_meta_object::Call,
        arg2: ::std::os::raw::c_int,
        arg3: *mut *mut ::std::ffi::c_void,
    ) -> ::std::os::raw::c_int;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineClientCertificateSelection_tr(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineClientCertificateSelection_trUtf8(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineClientCertificateSelection_slot_(
        this_ptr: *mut crate::SlotOfQWebEngineClientCertificateSelection,
        arg0: *const crate::QWebEngineClientCertificateSelection,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineClientCertificateSelection_staticMetaObject(
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineClientCertificateSelection_metaObject(
        this_ptr: *const crate::SignalOfQWebEngineClientCertificateSelection,
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineClientCertificateSelection_qt_metacast(
        this_ptr: *mut crate::SignalOfQWebEngineClientCertificateSelection,
        arg1: *const ::std::os::raw::c_char,
    ) -> *mut ::std::ffi::c_void;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineClientCertificateSelection_qt_metacall(
        this_ptr: *mut crate::SignalOfQWebEngineClientCertificateSelection,
        arg1: ::qt_core::q_meta_object::Call,
        arg2: ::std::os::raw::c_int,
        arg3: *mut *mut ::std::ffi::c_void,
    ) -> ::std::os::raw::c_int;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineClientCertificateSelection_tr(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineClientCertificateSelection_trUtf8(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineClientCertificateSelection_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineClientCertificateSelection(
    ) -> *mut crate::SignalOfQWebEngineClientCertificateSelection;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineClientCertificateSelection_emit_(
        this_ptr: *mut crate::SignalOfQWebEngineClientCertificateSelection,
        arg0: *const crate::QWebEngineClientCertificateSelection,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEnginePage_RenderProcessTerminationStatus_int_staticMetaObject(
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEnginePage_RenderProcessTerminationStatus_int_metaObject(
        this_ptr: *const crate::SlotOfRenderProcessTerminationStatusInt,
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEnginePage_RenderProcessTerminationStatus_int_qt_metacast(
        this_ptr: *mut crate::SlotOfRenderProcessTerminationStatusInt,
        arg1: *const ::std::os::raw::c_char,
    ) -> *mut ::std::ffi::c_void;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEnginePage_RenderProcessTerminationStatus_int_qt_metacall(
        this_ptr: *mut crate::SlotOfRenderProcessTerminationStatusInt,
        arg1: ::qt_core::q_meta_object::Call,
        arg2: ::std::os::raw::c_int,
        arg3: *mut *mut ::std::ffi::c_void,
    ) -> ::std::os::raw::c_int;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEnginePage_RenderProcessTerminationStatus_int_tr(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEnginePage_RenderProcessTerminationStatus_int_trUtf8(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEnginePage_RenderProcessTerminationStatus_int_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEnginePage_RenderProcessTerminationStatus_int(
        parent: *mut ::qt_core::QObject,
        callback: ::std::option::Option<
            extern "C" fn(
                *mut ::std::ffi::c_void,
                crate::q_web_engine_page::RenderProcessTerminationStatus,
                ::std::os::raw::c_int,
            ),
        >,
        deleter: ::std::option::Option<extern "C" fn(*mut ::std::ffi::c_void)>,
        data: *mut ::std::ffi::c_void,
    ) -> *mut crate::SlotOfRenderProcessTerminationStatusInt;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEnginePage_RenderProcessTerminationStatus_int_set(
        this_ptr: *mut crate::SlotOfRenderProcessTerminationStatusInt,
        callback: ::std::option::Option<
            extern "C" fn(
                *mut ::std::ffi::c_void,
                crate::q_web_engine_page::RenderProcessTerminationStatus,
                ::std::os::raw::c_int,
            ),
        >,
        deleter: ::std::option::Option<extern "C" fn(*mut ::std::ffi::c_void)>,
        data: *mut ::std::ffi::c_void,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEnginePage_RenderProcessTerminationStatus_int_slot_(
        this_ptr: *mut crate::SlotOfRenderProcessTerminationStatusInt,
        arg0: crate::q_web_engine_page::RenderProcessTerminationStatus,
        arg1: ::std::os::raw::c_int,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEnginePage_RenderProcessTerminationStatus_int_staticMetaObject(
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEnginePage_RenderProcessTerminationStatus_int_metaObject(
        this_ptr: *const crate::SignalOfRenderProcessTerminationStatusInt,
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEnginePage_RenderProcessTerminationStatus_int_qt_metacast(
        this_ptr: *mut crate::SignalOfRenderProcessTerminationStatusInt,
        arg1: *const ::std::os::raw::c_char,
    ) -> *mut ::std::ffi::c_void;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEnginePage_RenderProcessTerminationStatus_int_qt_metacall(
        this_ptr: *mut crate::SignalOfRenderProcessTerminationStatusInt,
        arg1: ::qt_core::q_meta_object::Call,
        arg2: ::std::os::raw::c_int,
        arg3: *mut *mut ::std::ffi::c_void,
    ) -> ::std::os::raw::c_int;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEnginePage_RenderProcessTerminationStatus_int_tr(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEnginePage_RenderProcessTerminationStatus_int_trUtf8(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEnginePage_RenderProcessTerminationStatus_int_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEnginePage_RenderProcessTerminationStatus_int(
    ) -> *mut crate::SignalOfRenderProcessTerminationStatusInt;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEnginePage_RenderProcessTerminationStatus_int_emit_(
        this_ptr: *mut crate::SignalOfRenderProcessTerminationStatusInt,
        arg0: crate::q_web_engine_page::RenderProcessTerminationStatus,
        arg1: ::std::os::raw::c_int,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_qint64_qint64_staticMetaObject(
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_qint64_qint64_metaObject(
        this_ptr: *const crate::SlotOfI64I64,
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_qint64_qint64_qt_metacast(
        this_ptr: *mut crate::SlotOfI64I64,
        arg1: *const ::std::os::raw::c_char,
    ) -> *mut ::std::ffi::c_void;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_qint64_qint64_qt_metacall(
        this_ptr: *mut crate::SlotOfI64I64,
        arg1: ::qt_core::q_meta_object::Call,
        arg2: ::std::os::raw::c_int,
        arg3: *mut *mut ::std::ffi::c_void,
    ) -> ::std::os::raw::c_int;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_qint64_qint64_tr(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_qint64_qint64_trUtf8(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_qint64_qint64_ctr_qt_web_engine_widgets_ffi_slot_wrapper_qint64_qint64(
        parent: *mut ::qt_core::QObject,
        callback: ::std::option::Option<
            extern "C" fn(
                *mut ::std::ffi::c_void,
                ::std::os::raw::c_longlong,
                ::std::os::raw::c_longlong,
            ),
        >,
        deleter: ::std::option::Option<extern "C" fn(*mut ::std::ffi::c_void)>,
        data: *mut ::std::ffi::c_void,
    ) -> *mut crate::SlotOfI64I64;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_qint64_qint64_set(
        this_ptr: *mut crate::SlotOfI64I64,
        callback: ::std::option::Option<
            extern "C" fn(
                *mut ::std::ffi::c_void,
                ::std::os::raw::c_longlong,
                ::std::os::raw::c_longlong,
            ),
        >,
        deleter: ::std::option::Option<extern "C" fn(*mut ::std::ffi::c_void)>,
        data: *mut ::std::ffi::c_void,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_qint64_qint64_slot_(
        this_ptr: *mut crate::SlotOfI64I64,
        arg0: i64,
        arg1: i64,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_qint64_qint64_staticMetaObject(
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_qint64_qint64_metaObject(
        this_ptr: *const crate::SignalOfI64I64,
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_qint64_qint64_qt_metacast(
        this_ptr: *mut crate::SignalOfI64I64,
        arg1: *const ::std::os::raw::c_char,
    ) -> *mut ::std::ffi::c_void;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_qint64_qint64_qt_metacall(
        this_ptr: *mut crate::SignalOfI64I64,
        arg1: ::qt_core::q_meta_object::Call,
        arg2: ::std::os::raw::c_int,
        arg3: *mut *mut ::std::ffi::c_void,
    ) -> ::std::os::raw::c_int;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_qint64_qint64_tr(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_qint64_qint64_trUtf8(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_qint64_qint64_ctr_qt_web_engine_widgets_ffi_signal_wrapper_qint64_qint64(
    ) -> *mut crate::SignalOfI64I64;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_qint64_qint64_emit_(
        this_ptr: *mut crate::SignalOfI64I64,
        arg0: i64,
        arg1: i64,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QUrl_const_ref_QWebEnginePage_Feature_staticMetaObject(
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QUrl_const_ref_QWebEnginePage_Feature_metaObject(
        this_ptr: *const crate::SlotOfQUrlFeature,
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QUrl_const_ref_QWebEnginePage_Feature_qt_metacast(
        this_ptr: *mut crate::SlotOfQUrlFeature,
        arg1: *const ::std::os::raw::c_char,
    ) -> *mut ::std::ffi::c_void;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QUrl_const_ref_QWebEnginePage_Feature_qt_metacall(
        this_ptr: *mut crate::SlotOfQUrlFeature,
        arg1: ::qt_core::q_meta_object::Call,
        arg2: ::std::os::raw::c_int,
        arg3: *mut *mut ::std::ffi::c_void,
    ) -> ::std::os::raw::c_int;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QUrl_const_ref_QWebEnginePage_Feature_tr(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QUrl_const_ref_QWebEnginePage_Feature_trUtf8(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QUrl_const_ref_QWebEnginePage_Feature_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QUrl_const_ref_QWebEnginePage_Feature(
        parent: *mut ::qt_core::QObject,
        callback: ::std::option::Option<
            extern "C" fn(
                *mut ::std::ffi::c_void,
                *const ::qt_core::QUrl,
                crate::q_web_engine_page::Feature,
            ),
        >,
        deleter: ::std::option::Option<extern "C" fn(*mut ::std::ffi::c_void)>,
        data: *mut ::std::ffi::c_void,
    ) -> *mut crate::SlotOfQUrlFeature;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QUrl_const_ref_QWebEnginePage_Feature_set(
        this_ptr: *mut crate::SlotOfQUrlFeature,
        callback: ::std::option::Option<
            extern "C" fn(
                *mut ::std::ffi::c_void,
                *const ::qt_core::QUrl,
                crate::q_web_engine_page::Feature,
            ),
        >,
        deleter: ::std::option::Option<extern "C" fn(*mut ::std::ffi::c_void)>,
        data: *mut ::std::ffi::c_void,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QUrl_const_ref_QWebEnginePage_Feature_slot_(
        this_ptr: *mut crate::SlotOfQUrlFeature,
        arg0: *const ::qt_core::QUrl,
        arg1: crate::q_web_engine_page::Feature,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QUrl_const_ref_QWebEnginePage_Feature_staticMetaObject(
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QUrl_const_ref_QWebEnginePage_Feature_metaObject(
        this_ptr: *const crate::SignalOfQUrlFeature,
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QUrl_const_ref_QWebEnginePage_Feature_qt_metacast(
        this_ptr: *mut crate::SignalOfQUrlFeature,
        arg1: *const ::std::os::raw::c_char,
    ) -> *mut ::std::ffi::c_void;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QUrl_const_ref_QWebEnginePage_Feature_qt_metacall(
        this_ptr: *mut crate::SignalOfQUrlFeature,
        arg1: ::qt_core::q_meta_object::Call,
        arg2: ::std::os::raw::c_int,
        arg3: *mut *mut ::std::ffi::c_void,
    ) -> ::std::os::raw::c_int;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QUrl_const_ref_QWebEnginePage_Feature_tr(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QUrl_const_ref_QWebEnginePage_Feature_trUtf8(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QUrl_const_ref_QWebEnginePage_Feature_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QUrl_const_ref_QWebEnginePage_Feature(
    ) -> *mut crate::SignalOfQUrlFeature;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QUrl_const_ref_QWebEnginePage_Feature_emit_(
        this_ptr: *mut crate::SignalOfQUrlFeature,
        arg0: *const ::qt_core::QUrl,
        arg1: crate::q_web_engine_page::Feature,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEnginePage_LifecycleState_staticMetaObject(
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEnginePage_LifecycleState_metaObject(
        this_ptr: *const crate::SlotOfLifecycleState,
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEnginePage_LifecycleState_qt_metacast(
        this_ptr: *mut crate::SlotOfLifecycleState,
        arg1: *const ::std::os::raw::c_char,
    ) -> *mut ::std::ffi::c_void;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEnginePage_LifecycleState_qt_metacall(
        this_ptr: *mut crate::SlotOfLifecycleState,
        arg1: ::qt_core::q_meta_object::Call,
        arg2: ::std::os::raw::c_int,
        arg3: *mut *mut ::std::ffi::c_void,
    ) -> ::std::os::raw::c_int;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEnginePage_LifecycleState_tr(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEnginePage_LifecycleState_trUtf8(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEnginePage_LifecycleState_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEnginePage_LifecycleState(
        parent: *mut ::qt_core::QObject,
        callback: ::std::option::Option<
            extern "C" fn(*mut ::std::ffi::c_void, crate::q_web_engine_page::LifecycleState),
        >,
        deleter: ::std::option::Option<extern "C" fn(*mut ::std::ffi::c_void)>,
        data: *mut ::std::ffi::c_void,
    ) -> *mut crate::SlotOfLifecycleState;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEnginePage_LifecycleState_set(
        this_ptr: *mut crate::SlotOfLifecycleState,
        callback: ::std::option::Option<
            extern "C" fn(*mut ::std::ffi::c_void, crate::q_web_engine_page::LifecycleState),
        >,
        deleter: ::std::option::Option<extern "C" fn(*mut ::std::ffi::c_void)>,
        data: *mut ::std::ffi::c_void,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEnginePage_LifecycleState_slot_(
        this_ptr: *mut crate::SlotOfLifecycleState,
        arg0: crate::q_web_engine_page::LifecycleState,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEnginePage_LifecycleState_staticMetaObject(
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEnginePage_LifecycleState_metaObject(
        this_ptr: *const crate::SignalOfLifecycleState,
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEnginePage_LifecycleState_qt_metacast(
        this_ptr: *mut crate::SignalOfLifecycleState,
        arg1: *const ::std::os::raw::c_char,
    ) -> *mut ::std::ffi::c_void;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEnginePage_LifecycleState_qt_metacall(
        this_ptr: *mut crate::SignalOfLifecycleState,
        arg1: ::qt_core::q_meta_object::Call,
        arg2: ::std::os::raw::c_int,
        arg3: *mut *mut ::std::ffi::c_void,
    ) -> ::std::os::raw::c_int;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEnginePage_LifecycleState_tr(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEnginePage_LifecycleState_trUtf8(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEnginePage_LifecycleState_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEnginePage_LifecycleState(
    ) -> *mut crate::SignalOfLifecycleState;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEnginePage_LifecycleState_emit_(
        this_ptr: *mut crate::SignalOfLifecycleState,
        arg0: crate::q_web_engine_page::LifecycleState,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEnginePage_RenderProcessTerminationStatus_staticMetaObject(
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEnginePage_RenderProcessTerminationStatus_metaObject(
        this_ptr: *const crate::SlotOfRenderProcessTerminationStatus,
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEnginePage_RenderProcessTerminationStatus_qt_metacast(
        this_ptr: *mut crate::SlotOfRenderProcessTerminationStatus,
        arg1: *const ::std::os::raw::c_char,
    ) -> *mut ::std::ffi::c_void;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEnginePage_RenderProcessTerminationStatus_qt_metacall(
        this_ptr: *mut crate::SlotOfRenderProcessTerminationStatus,
        arg1: ::qt_core::q_meta_object::Call,
        arg2: ::std::os::raw::c_int,
        arg3: *mut *mut ::std::ffi::c_void,
    ) -> ::std::os::raw::c_int;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEnginePage_RenderProcessTerminationStatus_tr(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEnginePage_RenderProcessTerminationStatus_trUtf8(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEnginePage_RenderProcessTerminationStatus_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEnginePage_RenderProcessTerminationStatus(
        parent: *mut ::qt_core::QObject,
        callback: ::std::option::Option<
            extern "C" fn(
                *mut ::std::ffi::c_void,
                crate::q_web_engine_page::RenderProcessTerminationStatus,
            ),
        >,
        deleter: ::std::option::Option<extern "C" fn(*mut ::std::ffi::c_void)>,
        data: *mut ::std::ffi::c_void,
    ) -> *mut crate::SlotOfRenderProcessTerminationStatus;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEnginePage_RenderProcessTerminationStatus_set(
        this_ptr: *mut crate::SlotOfRenderProcessTerminationStatus,
        callback: ::std::option::Option<
            extern "C" fn(
                *mut ::std::ffi::c_void,
                crate::q_web_engine_page::RenderProcessTerminationStatus,
            ),
        >,
        deleter: ::std::option::Option<extern "C" fn(*mut ::std::ffi::c_void)>,
        data: *mut ::std::ffi::c_void,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEnginePage_RenderProcessTerminationStatus_slot_(
        this_ptr: *mut crate::SlotOfRenderProcessTerminationStatus,
        arg0: crate::q_web_engine_page::RenderProcessTerminationStatus,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEnginePage_RenderProcessTerminationStatus_staticMetaObject(
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEnginePage_RenderProcessTerminationStatus_metaObject(
        this_ptr: *const crate::SignalOfRenderProcessTerminationStatus,
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEnginePage_RenderProcessTerminationStatus_qt_metacast(
        this_ptr: *mut crate::SignalOfRenderProcessTerminationStatus,
        arg1: *const ::std::os::raw::c_char,
    ) -> *mut ::std::ffi::c_void;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEnginePage_RenderProcessTerminationStatus_qt_metacall(
        this_ptr: *mut crate::SignalOfRenderProcessTerminationStatus,
        arg1: ::qt_core::q_meta_object::Call,
        arg2: ::std::os::raw::c_int,
        arg3: *mut *mut ::std::ffi::c_void,
    ) -> ::std::os::raw::c_int;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEnginePage_RenderProcessTerminationStatus_tr(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEnginePage_RenderProcessTerminationStatus_trUtf8(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEnginePage_RenderProcessTerminationStatus_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEnginePage_RenderProcessTerminationStatus(
    ) -> *mut crate::SignalOfRenderProcessTerminationStatus;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEnginePage_RenderProcessTerminationStatus_emit_(
        this_ptr: *mut crate::SignalOfRenderProcessTerminationStatus,
        arg0: crate::q_web_engine_page::RenderProcessTerminationStatus,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineDownloadItem_DownloadState_staticMetaObject(
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineDownloadItem_DownloadState_metaObject(
        this_ptr: *const crate::SlotOfDownloadState,
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineDownloadItem_DownloadState_qt_metacast(
        this_ptr: *mut crate::SlotOfDownloadState,
        arg1: *const ::std::os::raw::c_char,
    ) -> *mut ::std::ffi::c_void;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineDownloadItem_DownloadState_qt_metacall(
        this_ptr: *mut crate::SlotOfDownloadState,
        arg1: ::qt_core::q_meta_object::Call,
        arg2: ::std::os::raw::c_int,
        arg3: *mut *mut ::std::ffi::c_void,
    ) -> ::std::os::raw::c_int;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineDownloadItem_DownloadState_tr(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineDownloadItem_DownloadState_trUtf8(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineDownloadItem_DownloadState_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineDownloadItem_DownloadState(
        parent: *mut ::qt_core::QObject,
        callback: ::std::option::Option<
            extern "C" fn(
                *mut ::std::ffi::c_void,
                crate::q_web_engine_download_item::DownloadState,
            ),
        >,
        deleter: ::std::option::Option<extern "C" fn(*mut ::std::ffi::c_void)>,
        data: *mut ::std::ffi::c_void,
    ) -> *mut crate::SlotOfDownloadState;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineDownloadItem_DownloadState_set(
        this_ptr: *mut crate::SlotOfDownloadState,
        callback: ::std::option::Option<
            extern "C" fn(
                *mut ::std::ffi::c_void,
                crate::q_web_engine_download_item::DownloadState,
            ),
        >,
        deleter: ::std::option::Option<extern "C" fn(*mut ::std::ffi::c_void)>,
        data: *mut ::std::ffi::c_void,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineDownloadItem_DownloadState_slot_(
        this_ptr: *mut crate::SlotOfDownloadState,
        arg0: crate::q_web_engine_download_item::DownloadState,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineDownloadItem_DownloadState_staticMetaObject(
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineDownloadItem_DownloadState_metaObject(
        this_ptr: *const crate::SignalOfDownloadState,
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineDownloadItem_DownloadState_qt_metacast(
        this_ptr: *mut crate::SignalOfDownloadState,
        arg1: *const ::std::os::raw::c_char,
    ) -> *mut ::std::ffi::c_void;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineDownloadItem_DownloadState_qt_metacall(
        this_ptr: *mut crate::SignalOfDownloadState,
        arg1: ::qt_core::q_meta_object::Call,
        arg2: ::std::os::raw::c_int,
        arg3: *mut *mut ::std::ffi::c_void,
    ) -> ::std::os::raw::c_int;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineDownloadItem_DownloadState_tr(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineDownloadItem_DownloadState_trUtf8(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineDownloadItem_DownloadState_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineDownloadItem_DownloadState(
    ) -> *mut crate::SignalOfDownloadState;

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineDownloadItem_DownloadState_emit_(
        this_ptr: *mut crate::SignalOfDownloadState,
        arg0: crate::q_web_engine_download_item::DownloadState,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineFullScreenRequest_dctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineFullScreenRequest(
        this_ptr: *mut crate::SlotOfQWebEngineFullScreenRequest,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineFullScreenRequest_dctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineFullScreenRequest(
        this_ptr: *mut crate::SignalOfQWebEngineFullScreenRequest,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QString_const_ref_bool_dctr_qt_web_engine_widgets_ffi_slot_wrapper_QString_const_ref_bool(
        this_ptr: *mut crate::SlotOfQStringBool,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QString_const_ref_bool_dctr_qt_web_engine_widgets_ffi_signal_wrapper_QString_const_ref_bool(
        this_ptr: *mut crate::SignalOfQStringBool,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineDownloadItem_ptr_dctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineDownloadItem_ptr(
        this_ptr: *mut crate::SlotOfQWebEngineDownloadItem,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineDownloadItem_ptr_dctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineDownloadItem_ptr(
        this_ptr: *mut crate::SignalOfQWebEngineDownloadItem,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineClientCertificateSelection_dctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineClientCertificateSelection(
        this_ptr: *mut crate::SlotOfQWebEngineClientCertificateSelection,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineClientCertificateSelection_dctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineClientCertificateSelection(
        this_ptr: *mut crate::SignalOfQWebEngineClientCertificateSelection,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEnginePage_RenderProcessTerminationStatus_int_dctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEnginePage_RenderProcessTerminationStatus_int(
        this_ptr: *mut crate::SlotOfRenderProcessTerminationStatusInt,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEnginePage_RenderProcessTerminationStatus_int_dctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEnginePage_RenderProcessTerminationStatus_int(
        this_ptr: *mut crate::SignalOfRenderProcessTerminationStatusInt,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_qint64_qint64_dctr_qt_web_engine_widgets_ffi_slot_wrapper_qint64_qint64(
        this_ptr: *mut crate::SlotOfI64I64,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_qint64_qint64_dctr_qt_web_engine_widgets_ffi_signal_wrapper_qint64_qint64(
        this_ptr: *mut crate::SignalOfI64I64,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QUrl_const_ref_QWebEnginePage_Feature_dctr_qt_web_engine_widgets_ffi_slot_wrapper_QUrl_const_ref_QWebEnginePage_Feature(
        this_ptr: *mut crate::SlotOfQUrlFeature,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QUrl_const_ref_QWebEnginePage_Feature_dctr_qt_web_engine_widgets_ffi_signal_wrapper_QUrl_const_ref_QWebEnginePage_Feature(
        this_ptr: *mut crate::SignalOfQUrlFeature,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEnginePage_LifecycleState_dctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEnginePage_LifecycleState(
        this_ptr: *mut crate::SlotOfLifecycleState,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEnginePage_LifecycleState_dctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEnginePage_LifecycleState(
        this_ptr: *mut crate::SignalOfLifecycleState,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEnginePage_RenderProcessTerminationStatus_dctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEnginePage_RenderProcessTerminationStatus(
        this_ptr: *mut crate::SlotOfRenderProcessTerminationStatus,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEnginePage_RenderProcessTerminationStatus_dctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEnginePage_RenderProcessTerminationStatus(
        this_ptr: *mut crate::SignalOfRenderProcessTerminationStatus,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineDownloadItem_DownloadState_dctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineDownloadItem_DownloadState(
        this_ptr: *mut crate::SlotOfDownloadState,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineDownloadItem_DownloadState_dctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineDownloadItem_DownloadState(
        this_ptr: *mut crate::SignalOfDownloadState,
    );

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineFullScreenRequest_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::SlotOfQWebEngineFullScreenRequest;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_QObject_ptr8(
        ptr: *mut crate::SlotOfQWebEngineFullScreenRequest,
    ) -> *mut ::qt_core::QObject;

    pub fn ctr_qt_web_engine_widgets_ffi_dynamic_cast_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineFullScreenRequest_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::SlotOfQWebEngineFullScreenRequest;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineFullScreenRequest_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::SignalOfQWebEngineFullScreenRequest;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_QObject_ptr9(
        ptr: *mut crate::SignalOfQWebEngineFullScreenRequest,
    ) -> *mut ::qt_core::QObject;

    pub fn ctr_qt_web_engine_widgets_ffi_dynamic_cast_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineFullScreenRequest_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::SignalOfQWebEngineFullScreenRequest;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QString_const_ref_bool_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::SlotOfQStringBool;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_QObject_ptr14(
        ptr: *mut crate::SlotOfQStringBool,
    ) -> *mut ::qt_core::QObject;

    pub fn ctr_qt_web_engine_widgets_ffi_dynamic_cast_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QString_const_ref_bool_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::SlotOfQStringBool;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QString_const_ref_bool_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::SignalOfQStringBool;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_QObject_ptr15(
        ptr: *mut crate::SignalOfQStringBool,
    ) -> *mut ::qt_core::QObject;

    pub fn ctr_qt_web_engine_widgets_ffi_dynamic_cast_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QString_const_ref_bool_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::SignalOfQStringBool;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineDownloadItem_ptr_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::SlotOfQWebEngineDownloadItem;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_QObject_ptr16(
        ptr: *mut crate::SlotOfQWebEngineDownloadItem,
    ) -> *mut ::qt_core::QObject;

    pub fn ctr_qt_web_engine_widgets_ffi_dynamic_cast_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineDownloadItem_ptr_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::SlotOfQWebEngineDownloadItem;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineDownloadItem_ptr_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::SignalOfQWebEngineDownloadItem;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_QObject_ptr17(
        ptr: *mut crate::SignalOfQWebEngineDownloadItem,
    ) -> *mut ::qt_core::QObject;

    pub fn ctr_qt_web_engine_widgets_ffi_dynamic_cast_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineDownloadItem_ptr_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::SignalOfQWebEngineDownloadItem;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineClientCertificateSelection_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::SlotOfQWebEngineClientCertificateSelection;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_QObject_ptr18(
        ptr: *mut crate::SlotOfQWebEngineClientCertificateSelection,
    ) -> *mut ::qt_core::QObject;

    pub fn ctr_qt_web_engine_widgets_ffi_dynamic_cast_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineClientCertificateSelection_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::SlotOfQWebEngineClientCertificateSelection;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineClientCertificateSelection_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::SignalOfQWebEngineClientCertificateSelection;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_QObject_ptr19(
        ptr: *mut crate::SignalOfQWebEngineClientCertificateSelection,
    ) -> *mut ::qt_core::QObject;

    pub fn ctr_qt_web_engine_widgets_ffi_dynamic_cast_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineClientCertificateSelection_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::SignalOfQWebEngineClientCertificateSelection;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEnginePage_RenderProcessTerminationStatus_int_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::SlotOfRenderProcessTerminationStatusInt;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_QObject_ptr20(
        ptr: *mut crate::SlotOfRenderProcessTerminationStatusInt,
    ) -> *mut ::qt_core::QObject;

    pub fn ctr_qt_web_engine_widgets_ffi_dynamic_cast_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEnginePage_RenderProcessTerminationStatus_int_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::SlotOfRenderProcessTerminationStatusInt;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEnginePage_RenderProcessTerminationStatus_int_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::SignalOfRenderProcessTerminationStatusInt;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_QObject_ptr21(
        ptr: *mut crate::SignalOfRenderProcessTerminationStatusInt,
    ) -> *mut ::qt_core::QObject;

    pub fn ctr_qt_web_engine_widgets_ffi_dynamic_cast_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEnginePage_RenderProcessTerminationStatus_int_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::SignalOfRenderProcessTerminationStatusInt;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_ctr_qt_web_engine_widgets_ffi_slot_wrapper_qint64_qint64_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::SlotOfI64I64;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_QObject_ptr22(
        ptr: *mut crate::SlotOfI64I64,
    ) -> *mut ::qt_core::QObject;

    pub fn ctr_qt_web_engine_widgets_ffi_dynamic_cast_ctr_qt_web_engine_widgets_ffi_slot_wrapper_qint64_qint64_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::SlotOfI64I64;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_ctr_qt_web_engine_widgets_ffi_signal_wrapper_qint64_qint64_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::SignalOfI64I64;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_QObject_ptr23(
        ptr: *mut crate::SignalOfI64I64,
    ) -> *mut ::qt_core::QObject;

    pub fn ctr_qt_web_engine_widgets_ffi_dynamic_cast_ctr_qt_web_engine_widgets_ffi_signal_wrapper_qint64_qint64_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::SignalOfI64I64;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QUrl_const_ref_QWebEnginePage_Feature_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::SlotOfQUrlFeature;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_QObject_ptr24(
        ptr: *mut crate::SlotOfQUrlFeature,
    ) -> *mut ::qt_core::QObject;

    pub fn ctr_qt_web_engine_widgets_ffi_dynamic_cast_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QUrl_const_ref_QWebEnginePage_Feature_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::SlotOfQUrlFeature;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QUrl_const_ref_QWebEnginePage_Feature_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::SignalOfQUrlFeature;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_QObject_ptr25(
        ptr: *mut crate::SignalOfQUrlFeature,
    ) -> *mut ::qt_core::QObject;

    pub fn ctr_qt_web_engine_widgets_ffi_dynamic_cast_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QUrl_const_ref_QWebEnginePage_Feature_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::SignalOfQUrlFeature;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEnginePage_LifecycleState_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::SlotOfLifecycleState;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_QObject_ptr26(
        ptr: *mut crate::SlotOfLifecycleState,
    ) -> *mut ::qt_core::QObject;

    pub fn ctr_qt_web_engine_widgets_ffi_dynamic_cast_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEnginePage_LifecycleState_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::SlotOfLifecycleState;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEnginePage_LifecycleState_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::SignalOfLifecycleState;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_QObject_ptr27(
        ptr: *mut crate::SignalOfLifecycleState,
    ) -> *mut ::qt_core::QObject;

    pub fn ctr_qt_web_engine_widgets_ffi_dynamic_cast_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEnginePage_LifecycleState_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::SignalOfLifecycleState;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEnginePage_RenderProcessTerminationStatus_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::SlotOfRenderProcessTerminationStatus;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_QObject_ptr30(
        ptr: *mut crate::SlotOfRenderProcessTerminationStatus,
    ) -> *mut ::qt_core::QObject;

    pub fn ctr_qt_web_engine_widgets_ffi_dynamic_cast_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEnginePage_RenderProcessTerminationStatus_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::SlotOfRenderProcessTerminationStatus;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEnginePage_RenderProcessTerminationStatus_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::SignalOfRenderProcessTerminationStatus;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_QObject_ptr31(
        ptr: *mut crate::SignalOfRenderProcessTerminationStatus,
    ) -> *mut ::qt_core::QObject;

    pub fn ctr_qt_web_engine_widgets_ffi_dynamic_cast_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEnginePage_RenderProcessTerminationStatus_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::SignalOfRenderProcessTerminationStatus;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineDownloadItem_DownloadState_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::SlotOfDownloadState;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_QObject_ptr32(
        ptr: *mut crate::SlotOfDownloadState,
    ) -> *mut ::qt_core::QObject;

    pub fn ctr_qt_web_engine_widgets_ffi_dynamic_cast_ctr_qt_web_engine_widgets_ffi_slot_wrapper_QWebEngineDownloadItem_DownloadState_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::SlotOfDownloadState;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineDownloadItem_DownloadState_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::SignalOfDownloadState;

    pub fn ctr_qt_web_engine_widgets_ffi_static_cast_QObject_ptr33(
        ptr: *mut crate::SignalOfDownloadState,
    ) -> *mut ::qt_core::QObject;

    pub fn ctr_qt_web_engine_widgets_ffi_dynamic_cast_ctr_qt_web_engine_widgets_ffi_signal_wrapper_QWebEngineDownloadItem_DownloadState_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::SignalOfDownloadState;

}
