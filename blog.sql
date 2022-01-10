CREATE TABLE blog_article (
  `id` int(11) unsigned NOT NULL AUTO_INCREMENT,
  `createTime` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `modifyTime` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '修改时间',
  `detailLastModifyTime` datetime NOT NULL DEFAULT '1970-01-01 00:00:00' COMMENT '文章详情最近更新时间',
  `title` varchar(50) NOT NULL DEFAULT '' COMMENT '文章标题',
  `authorId` int(11) unsigned NOT NULL DEFAULT '0' COMMENT '作者id',
  `likes` int(10) unsigned NOT NULL DEFAULT '0' COMMENT '点赞数',
  `readCount` int(10) unsigned NOT NULL DEFAULT '0' COMMENT '已读人数',
  `isHidden` tinyint(3) unsigned NOT NULL DEFAULT '1' COMMENT '隐藏 是1、否0,隐藏只后台可见',
  `coverImageUrl` varchar(128) NOT NULL DEFAULT '' COMMENT '封面图片',
  `categoryId` int(11) unsigned NOT NULL DEFAULT '0' COMMENT '分类编号',
  `summary` varchar(1024) NOT NULL DEFAULT '' COMMENT '摘要总结 - 支持多段',
  `coverUrls` varchar(300) NOT NULL DEFAULT '' COMMENT '多个主图',
  `shareCount` int(10) NOT NULL DEFAULT '0' COMMENT '分享次数',
  `collections` int(11) NOT NULL DEFAULT '0' COMMENT '收藏总数',
  `comments` int(11) NOT NULL DEFAULT '0' COMMENT '评论总数',
  `stick` bigint(20) unsigned NOT NULL DEFAULT '0' COMMENT '是否置顶,置顶排序值 0为不置顶,时间戳为置顶时间',
  PRIMARY KEY (`id`),
  KEY `idx_title` (`title`),
  KEY `idx_detailLastModifyTime` (`detailLastModifyTime`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='文章';

CREATE TABLE blog_article_content (
  `id` int(11) unsigned NOT NULL AUTO_INCREMENT,
  `createTime` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `modifyTime` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '修改时间',
  `articleId` int(11) unsigned NOT NULL DEFAULT '0' COMMENT '文章id',
  `content`  mediumtext COMMENT '文章内容',
  PRIMARY KEY (`id`),
  KEY `idx_articleId` (`articleId`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='文章内容';

CREATE TABLE `blog_article_reply` (
  `id` int(11) unsigned NOT NULL DEFAULT '0' COMMENT '全局id',
  `createTime` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `modifyTime` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '修改时间',
  `content` text COMMENT '回复内容',
  `articleId` int(11) unsigned NOT NULL DEFAULT '0' COMMENT '评论id',
  `parentReplyId` int(11) unsigned NOT NULL DEFAULT '0' COMMENT '上级回复id',
  `floor` int(11) unsigned NOT NULL DEFAULT '0' COMMENT '回复楼层，从1开始',
  `userId` int(11) unsigned NOT NULL DEFAULT '0' COMMENT '回复人id',
  `ip` varchar(64) NOT NULL DEFAULT '' COMMENT '发表内容时的IP地址',
  `status` tinyint(3) unsigned NOT NULL DEFAULT '0' COMMENT '状态',
  PRIMARY KEY (`id`),
  KEY `idx_articleId` (`articleId`),
  KEY `idx_userId` (`userId`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='评论回复';

CREATE TABLE `blog_message` (
  `id` int(11) unsigned NOT NULL DEFAULT '0' COMMENT '全局id',
  `createTime` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `modifyTime` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '修改时间',
  `toUserId` int(11) unsigned NOT NULL DEFAULT '0' COMMENT '接收用户id',
  `fromUserId` int(11) unsigned NOT NULL DEFAULT '0' COMMENT '发表当前内容的用户编号',
  `admin` tinyint(3) unsigned NOT NULL DEFAULT '0' COMMENT '是否为系统发送',
  `referId` int(11) unsigned NOT NULL DEFAULT '0' COMMENT '当前讨论的实体id',
  `referType` int(10) unsigned NOT NULL DEFAULT '0' COMMENT '实体类型，全局编号',
  `referDelete` tinyint(3) unsigned NOT NULL DEFAULT '0' COMMENT '引用是否删除',
  `type` int(10) unsigned NOT NULL DEFAULT '0' COMMENT '消息类型',
  `content` text COMMENT '消息内容',
  `referContent` text COMMENT '引用的内容',
  `haveRead` tinyint(3) unsigned NOT NULL DEFAULT '0' COMMENT '是否已读：已读1，未读0',
  PRIMARY KEY (`id`),
  KEY `idx_toUserId` (`toUserId`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='消息';
