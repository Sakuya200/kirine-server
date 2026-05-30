create table if not exists app_meta
(
    key   varchar(20)       not null
        primary key,
    value varchar(512) not null
);

alter table app_meta
    owner to postgres;

create table if not exists model_info
(
    id                               bigserial
        primary key,
    base_model                       varchar(100)                                     not null,
    model_name                       varchar(100)                                     not null,
    model_version                    varchar(100)                                     not null,
    download_type                    varchar(20) default 'HF-Like'::character varying not null,
    required_model_name_list_json    text                                             not null,
    required_model_repo_id_list_json text                                             not null,
    supported_feature_list_json      text                                             not null,
    supported_devices                text        default '[]'::text                   not null,
    create_time                      timestamp                                        not null,
    modify_time                      timestamp                                        not null,
    downloaded                       boolean     default false                        not null,
    deleted                          smallint    default 0                            not null
);

alter table model_info
    owner to postgres;

create index if not exists downloaded_index
    on model_info (downloaded);

create index if not exists model_info_base_model_index
    on model_info (base_model);

create index if not exists model_info_download_type_index
    on model_info (download_type);

create table if not exists model_training_tasks
(
    id                bigserial
        primary key,
    history_id        bigint                      not null,
    language          varchar(20)                 not null,
    base_model        varchar(100)                not null,
    model_version     varchar(100)                not null,
    speaker_name      varchar(20)                 not null,
    model_params_json text     default '{}'::text not null,
    sample_count      bigint                      not null,
    samples_json      text     default '[]'::text not null,
    notes_json        text                        not null,
    output_speaker_id bigint,
    description       text     default ''::text   not null,
    create_time       timestamp                   not null,
    modify_time       timestamp                   not null,
    deleted           smallint default 0          not null
);

alter table model_training_tasks
    owner to postgres;

create index if not exists model_training_tasks_output_speaker_id_index
    on model_training_tasks (output_speaker_id);

create index if not exists model_training_tasks_history_id_index
    on model_training_tasks (history_id);

create index if not exists model_training_tasks_modify_time_index
    on model_training_tasks (modify_time);

create table if not exists seaql_migrations
(
    version    varchar(100) not null
        primary key,
    applied_at bigint       not null
);

alter table seaql_migrations
    owner to postgres;

create table if not exists speakers
(
    id             bigserial
        primary key,
    name           varchar(20)               not null,
    languages_json text                      not null,
    samples        bigint   default 0        not null,
    base_model     varchar(100)              not null,
    description    text     default ''::text not null,
    status         varchar(20)               not null,
    source         varchar(20)               not null,
    create_time    timestamp                 not null,
    modify_time    timestamp                 not null,
    deleted        smallint default 0        not null
);

alter table speakers
    owner to postgres;

create index if not exists speakers_base_model_index
    on speakers (base_model);

create index if not exists speakers_modify_time_index
    on speakers (modify_time);

create table if not exists task_history
(
    id               bigserial
        primary key,
    task_type        varchar(20)                                  not null,
    title            varchar(100)                                 not null,
    speaker_id       bigint,
    speaker_name     varchar(20)                                  not null,
    status           varchar(20)                                  not null,
    duration_seconds bigint      default 0                        not null,
    create_time      timestamp                                    not null,
    modify_time      timestamp                                    not null,
    finished_time    timestamp,
    device           varchar(20) default 'cpu'::character varying not null,
    deleted          smallint    default 0                        not null,
    base_model       varchar(100)                                 not null
);

alter table task_history
    owner to postgres;

create index if not exists task_history_base_model_index
    on task_history (base_model);

create index if not exists task_history_task_type_index
    on task_history (task_type);

create index if not exists task_history_modify_time_index
    on task_history (modify_time);

create table if not exists tts_tasks
(
    id                bigint   default nextval('tts_task_id_seq'::regclass) not null
        primary key,
    history_id        bigint                                                not null,
    speaker_id        bigint,
    model_path        text,
    base_model        varchar(100)                                          not null,
    model_version     varchar(100)                                          not null,
    language          varchar(20)                                           not null,
    format            varchar(10)                                           not null,
    export_audio_name varchar(100)                                          not null,
    text              text                                                  not null,
    model_params_json text     default '{}'::text                           not null,
    char_count        integer                                               not null,
    output_file_path  text,
    create_time       timestamp                                             not null,
    modify_time       timestamp                                             not null,
    deleted           smallint default 0                                    not null
);

alter table tts_tasks
    owner to postgres;

create index if not exists tts_tasks_history_id_index
    on tts_tasks (history_id);

create table if not exists voice_clone_tasks
(
    id                bigserial
        primary key,
    history_id        bigint                                       not null,
    base_model        varchar(100)                                 not null,
    model_version     varchar(100)                                 not null,
    language          varchar(20)                                  not null,
    format            varchar(10) default 'wav'::character varying not null,
    export_audio_name varchar(100)                                 not null,
    ref_audio_name    varchar(100)                                 not null,
    ref_audio_path    text                                         not null,
    ref_text          text                                         not null,
    text              text                                         not null,
    model_params_json text        default '{}'::text               not null,
    char_count        integer                                      not null,
    output_file_path  text,
    create_time       timestamp                                    not null,
    modify_time       timestamp                                    not null,
    deleted           smallint    default 0                        not null
);

alter table voice_clone_tasks
    owner to postgres;

create index if not exists voice_clone_tasks_history_id_index
    on voice_clone_tasks (history_id);


