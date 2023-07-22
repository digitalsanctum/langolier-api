--
-- PostgreSQL database dump
--

-- Dumped from database version 15.3 (Ubuntu 15.3-1.pgdg22.04+1)
-- Dumped by pg_dump version 15.3 (Ubuntu 15.3-1.pgdg22.04+1)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: company; Type: TABLE; Schema: public; Owner: shane
--

CREATE TABLE public.company (
                                id uuid DEFAULT gen_random_uuid() NOT NULL,
                                name character varying(100) NOT NULL,
                                url text,
                                ticker character varying(10),
                                indeed_rating character varying(10),
                                glassdoor_rating character varying(10),
                                create_timestamp timestamp with time zone DEFAULT now() NOT NULL,
                                update_timestamp timestamp with time zone DEFAULT now() NOT NULL,
                                num_employees_min integer,
                                num_employees_max integer,
                                sector character varying(50),
                                industry character varying(50),
                                exchange character varying(20),
                                address text
);


ALTER TABLE public.company OWNER TO shane;

--
-- Name: company_rating; Type: TABLE; Schema: public; Owner: shane
--

CREATE TABLE public.company_rating (
                                       id uuid DEFAULT gen_random_uuid() NOT NULL,
                                       company_id uuid NOT NULL,
                                       rating_id uuid NOT NULL,
                                       create_timestamp timestamp with time zone DEFAULT now(),
                                       update_timestamp timestamp with time zone DEFAULT now(),
                                       rating_value numeric NOT NULL
);


ALTER TABLE public.company_rating OWNER TO shane;

--
-- Name: company_urls; Type: TABLE; Schema: public; Owner: shane
--

CREATE TABLE public.company_urls (
                                     id uuid DEFAULT gen_random_uuid() NOT NULL,
                                     url_type uuid NOT NULL
);


ALTER TABLE public.company_urls OWNER TO shane;

--
-- Name: dataset; Type: TABLE; Schema: public; Owner: shane
--

CREATE TABLE public.dataset (
                                id uuid DEFAULT gen_random_uuid() NOT NULL,
                                name character varying(255),
                                type character varying(50)
);


ALTER TABLE public.dataset OWNER TO shane;

--
-- Name: dataset_item; Type: TABLE; Schema: public; Owner: shane
--

CREATE TABLE public.dataset_item (
                                     id uuid DEFAULT gen_random_uuid() NOT NULL,
                                     name character varying(255),
                                     value text
);


ALTER TABLE public.dataset_item OWNER TO shane;

--
-- Name: feed; Type: TABLE; Schema: public; Owner: shane
--

CREATE TABLE public.feed (
                             id uuid DEFAULT gen_random_uuid() NOT NULL,
                             url text NOT NULL,
                             source_id uuid NOT NULL,
                             feed_type character varying(100),
                             ttl integer,
                             title text,
                             create_timestamp timestamp with time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.feed OWNER TO shane;

--
-- Name: garden; Type: TABLE; Schema: public; Owner: shane
--

CREATE TABLE public.garden (
                               id uuid DEFAULT gen_random_uuid() NOT NULL,
                               title text NOT NULL,
                               create_timestamp timestamp with time zone DEFAULT now() NOT NULL,
                               slug text NOT NULL,
                               update_timestamp timestamp with time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.garden OWNER TO shane;

--
-- Name: link; Type: TABLE; Schema: public; Owner: shane
--

CREATE TABLE public.link (
                             id uuid DEFAULT gen_random_uuid() NOT NULL,
                             source_page_id uuid NOT NULL,
                             target_page_id uuid NOT NULL,
                             label text
);


ALTER TABLE public.link OWNER TO shane;

--
-- Name: news; Type: TABLE; Schema: public; Owner: shane
--

CREATE TABLE public.news (
                             id uuid DEFAULT gen_random_uuid() NOT NULL,
                             title text NOT NULL,
                             url text NOT NULL,
                             published_timestamp timestamp with time zone NOT NULL,
                             guid character varying(255) NOT NULL,
                             feed_id uuid NOT NULL,
                             create_timestamp timestamp with time zone DEFAULT now() NOT NULL,
                             raw_content_path text,
                             text_content_path text
);


ALTER TABLE public.news OWNER TO shane;

--
-- Name: page; Type: TABLE; Schema: public; Owner: shane
--

CREATE TABLE public.page (
                             id uuid DEFAULT gen_random_uuid() NOT NULL,
                             title text NOT NULL,
                             content text NOT NULL,
                             garden_id uuid NOT NULL,
                             create_timestamp timestamp with time zone DEFAULT now() NOT NULL,
                             published boolean DEFAULT false NOT NULL,
                             slug text NOT NULL,
                             update_timestamp timestamp with time zone DEFAULT now() NOT NULL,
                             page_type character varying(50) NOT NULL
);


ALTER TABLE public.page OWNER TO shane;

--
-- Name: page_tag; Type: TABLE; Schema: public; Owner: shane
--

CREATE TABLE public.page_tag (
                                 id uuid DEFAULT gen_random_uuid() NOT NULL,
                                 page_id uuid NOT NULL,
                                 tag_id uuid NOT NULL
);


ALTER TABLE public.page_tag OWNER TO shane;

--
-- Name: person; Type: TABLE; Schema: public; Owner: shane
--

CREATE TABLE public.person (
                               id uuid DEFAULT gen_random_uuid() NOT NULL,
                               first_name character varying(50) NOT NULL,
                               last_name character varying(50) NOT NULL
);


ALTER TABLE public.person OWNER TO shane;

--
-- Name: rating; Type: TABLE; Schema: public; Owner: shane
--

CREATE TABLE public.rating (
                               id uuid DEFAULT gen_random_uuid() NOT NULL,
                               name character varying(100),
                               min_value numeric,
                               max_value numeric
);


ALTER TABLE public.rating OWNER TO shane;

--
-- Name: scope; Type: TABLE; Schema: public; Owner: shane
--

CREATE TABLE public.scope (
                              id uuid DEFAULT gen_random_uuid() NOT NULL,
                              name character varying(100) NOT NULL,
                              description text,
                              create_timestamp timestamp with time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.scope OWNER TO shane;

--
-- Name: source; Type: TABLE; Schema: public; Owner: shane
--

CREATE TABLE public.source (
                               id uuid DEFAULT gen_random_uuid() NOT NULL,
                               name character varying(100) NOT NULL,
                               url character varying(255) NOT NULL,
                               city character varying(100),
                               state character varying(100),
                               short_name character varying(100),
                               description text,
                               create_timestamp timestamp with time zone DEFAULT now() NOT NULL,
                               feed_available boolean,
                               paywall boolean,
                               type_id integer NOT NULL
);


ALTER TABLE public.source OWNER TO shane;

--
-- Name: source_change; Type: TABLE; Schema: public; Owner: shane
--

CREATE TABLE public.source_change (
                                      id uuid DEFAULT gen_random_uuid() NOT NULL,
                                      source_id uuid NOT NULL,
                                      change_timestamp timestamp with time zone DEFAULT now(),
                                      content_before text,
                                      content_after text
);


ALTER TABLE public.source_change OWNER TO shane;

--
-- Name: source_distribution; Type: TABLE; Schema: public; Owner: shane
--

CREATE TABLE public.source_distribution (
                                            id uuid DEFAULT gen_random_uuid() NOT NULL,
                                            source_id uuid,
                                            city_id uuid,
                                            zip_locale_id uuid
);


ALTER TABLE public.source_distribution OWNER TO shane;

--
-- Name: source_type; Type: TABLE; Schema: public; Owner: shane
--

CREATE TABLE public.source_type (
                                    name character varying(100) NOT NULL,
                                    id integer NOT NULL
);


ALTER TABLE public.source_type OWNER TO shane;

--
-- Name: source_type_id_seq; Type: SEQUENCE; Schema: public; Owner: shane
--

CREATE SEQUENCE public.source_type_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.source_type_id_seq OWNER TO shane;

--
-- Name: source_type_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: shane
--

ALTER SEQUENCE public.source_type_id_seq OWNED BY public.source_type.id;


--
-- Name: tag; Type: TABLE; Schema: public; Owner: shane
--

CREATE TABLE public.tag (
                            id uuid DEFAULT gen_random_uuid() NOT NULL,
                            name character varying(100) NOT NULL
);


ALTER TABLE public.tag OWNER TO shane;

--
-- Name: tool; Type: TABLE; Schema: public; Owner: shane
--

CREATE TABLE public.tool (
                             id uuid DEFAULT gen_random_uuid() NOT NULL,
                             name text NOT NULL,
                             create_timestamp timestamp with time zone DEFAULT now() NOT NULL,
                             update_timestamp timestamp with time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.tool OWNER TO shane;

--
-- Name: urls; Type: TABLE; Schema: public; Owner: shane
--

CREATE TABLE public.urls (
                             id uuid DEFAULT gen_random_uuid() NOT NULL,
                             title text,
                             url text
);


ALTER TABLE public.urls OWNER TO shane;

--
-- Name: workspace; Type: TABLE; Schema: public; Owner: shane
--

CREATE TABLE public.workspace (
                                  id uuid DEFAULT gen_random_uuid() NOT NULL,
                                  name character varying(100) NOT NULL,
                                  description text,
                                  create_timestamp timestamp with time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.workspace OWNER TO shane;

--
-- Name: source_type id; Type: DEFAULT; Schema: public; Owner: shane
--

ALTER TABLE ONLY public.source_type ALTER COLUMN id SET DEFAULT nextval('public.source_type_id_seq'::regclass);


--
-- Data for Name: company; Type: TABLE DATA; Schema: public; Owner: shane
--

INSERT INTO public.company (id, name, url, ticker, indeed_rating, glassdoor_rating, create_timestamp, update_timestamp, num_employees_min, num_employees_max, sector, industry, exchange, address) VALUES ('cd14040e-89e6-4454-a5fa-5f4ed01f89d9', 'Google', 'https://google.com', NULL, NULL, NULL, '2023-07-02 12:23:01.876978-07', '2023-07-02 12:23:01.876978-07', NULL, NULL, NULL, NULL, NULL, NULL);
INSERT INTO public.company (id, name, url, ticker, indeed_rating, glassdoor_rating, create_timestamp, update_timestamp, num_employees_min, num_employees_max, sector, industry, exchange, address) VALUES ('0f51976d-faff-4323-b7b0-9eb42287d4cc', 'Apple', 'https://apple.com', NULL, NULL, NULL, '2023-07-02 12:23:11.485155-07', '2023-07-02 12:23:11.485155-07', NULL, NULL, NULL, NULL, NULL, NULL);
INSERT INTO public.company (id, name, url, ticker, indeed_rating, glassdoor_rating, create_timestamp, update_timestamp, num_employees_min, num_employees_max, sector, industry, exchange, address) VALUES ('eaeade65-797f-40bb-8230-13c1c792b45f', 'Yahoo', 'https://yahoo.com', NULL, NULL, NULL, '2023-07-08 10:39:52.242696-07', '2023-07-08 10:39:52.242696-07', NULL, NULL, NULL, NULL, NULL, NULL);


--
-- Data for Name: company_rating; Type: TABLE DATA; Schema: public; Owner: shane
--



--
-- Data for Name: company_urls; Type: TABLE DATA; Schema: public; Owner: shane
--



--
-- Data for Name: dataset; Type: TABLE DATA; Schema: public; Owner: shane
--



--
-- Data for Name: dataset_item; Type: TABLE DATA; Schema: public; Owner: shane
--



--
-- Data for Name: feed; Type: TABLE DATA; Schema: public; Owner: shane
--



--
-- Data for Name: garden; Type: TABLE DATA; Schema: public; Owner: shane
--

INSERT INTO public.garden (id, title, create_timestamp, slug, update_timestamp) VALUES ('ea0c9b57-8aa3-42d4-a0a0-ea99c4c7f7d8', 'Shane''s Garden', '2023-07-03 06:48:10.717648-07', 'shanes-garden', '2023-07-03 06:48:10.717648-07');
INSERT INTO public.garden (id, title, create_timestamp, slug, update_timestamp) VALUES ('5ba011e0-15a2-41bd-9d6b-dbcccd994d3c', 'test', '2023-07-05 07:39:59.912104-07', 'test-slug', '2023-07-05 07:39:59.912104-07');
INSERT INTO public.garden (id, title, create_timestamp, slug, update_timestamp) VALUES ('49aba2f5-1458-4006-8489-de5425855653', 'test2', '2023-07-05 15:12:39.39977-07', 'tes2', '2023-07-05 15:12:39.39977-07');
INSERT INTO public.garden (id, title, create_timestamp, slug, update_timestamp) VALUES ('29663b59-4666-41fb-a851-bd1fee28639b', 'tester', '2023-07-05 15:12:56.060469-07', 'tester', '2023-07-05 15:12:56.060469-07');
INSERT INTO public.garden (id, title, create_timestamp, slug, update_timestamp) VALUES ('c677b10f-c4c4-4d71-a60b-1f358d1a30f7', 'test3', '2023-07-06 10:51:34.33531-07', 'test3-slug', '2023-07-06 10:51:34.33531-07');


--
-- Data for Name: link; Type: TABLE DATA; Schema: public; Owner: shane
--



--
-- Data for Name: news; Type: TABLE DATA; Schema: public; Owner: shane
--



--
-- Data for Name: page; Type: TABLE DATA; Schema: public; Owner: shane
--

INSERT INTO public.page (id, title, content, garden_id, create_timestamp, published, slug, update_timestamp, page_type) VALUES ('e01e9c8e-1291-4fc3-9d32-15ed3611fbcd', 'Test Page 1', '# Test Page 1
foo

bar', 'ea0c9b57-8aa3-42d4-a0a0-ea99c4c7f7d8', '2023-07-03 06:49:44.016226-07', false, 'test-page-1', '2023-07-02 23:49:40.937-07', 'sapling');
INSERT INTO public.page (id, title, content, garden_id, create_timestamp, published, slug, update_timestamp, page_type) VALUES ('ab7d5838-ae3b-403a-a9be-865222ce40c5', 'test', 'test', 'ea0c9b57-8aa3-42d4-a0a0-ea99c4c7f7d8', '2023-07-06 12:05:25.963705-07', false, 'test', '2023-07-06 12:05:25.963705-07', 'test');


--
-- Data for Name: page_tag; Type: TABLE DATA; Schema: public; Owner: shane
--



--
-- Data for Name: person; Type: TABLE DATA; Schema: public; Owner: shane
--



--
-- Data for Name: rating; Type: TABLE DATA; Schema: public; Owner: shane
--



--
-- Data for Name: scope; Type: TABLE DATA; Schema: public; Owner: shane
--

INSERT INTO public.scope (id, name, description, create_timestamp) VALUES ('71dfdf60-f1b0-44c3-a8b8-2064d818c0d7', 'internal', NULL, '2023-06-22 10:05:12.989324-07');
INSERT INTO public.scope (id, name, description, create_timestamp) VALUES ('a0aecdc8-97a2-4c75-ab8d-fb0322ebbec8', 'external', NULL, '2023-06-22 10:05:22.657032-07');


--
-- Data for Name: source; Type: TABLE DATA; Schema: public; Owner: shane
--



--
-- Data for Name: source_change; Type: TABLE DATA; Schema: public; Owner: shane
--



--
-- Data for Name: source_distribution; Type: TABLE DATA; Schema: public; Owner: shane
--



--
-- Data for Name: source_type; Type: TABLE DATA; Schema: public; Owner: shane
--

INSERT INTO public.source_type (name, id) VALUES ('Aggregator', 1);
INSERT INTO public.source_type (name, id) VALUES ('Local Newspaper', 2);
INSERT INTO public.source_type (name, id) VALUES ('Website', 3);
INSERT INTO public.source_type (name, id) VALUES ('Feed', 4);
INSERT INTO public.source_type (name, id) VALUES ('OPML File', 5);
INSERT INTO public.source_type (name, id) VALUES ('GitHub Repository', 6);
INSERT INTO public.source_type (name, id) VALUES ('CSV File', 7);
INSERT INTO public.source_type (name, id) VALUES ('Text File', 8);
INSERT INTO public.source_type (name, id) VALUES ('PostgreSQL Table', 9);
INSERT INTO public.source_type (name, id) VALUES ('Webpage', 13);
INSERT INTO public.source_type (name, id) VALUES ('Newsletter', 14);


--
-- Data for Name: tag; Type: TABLE DATA; Schema: public; Owner: shane
--



--
-- Data for Name: tool; Type: TABLE DATA; Schema: public; Owner: shane
--



--
-- Data for Name: urls; Type: TABLE DATA; Schema: public; Owner: shane
--



--
-- Data for Name: workspace; Type: TABLE DATA; Schema: public; Owner: shane
--

INSERT INTO public.workspace (id, name, description, create_timestamp) VALUES ('cd023261-4e9e-48e3-85c3-ad15d7694e21', 'test', NULL, '2023-06-22 10:02:07.686464-07');


--
-- Name: source_type_id_seq; Type: SEQUENCE SET; Schema: public; Owner: shane
--

SELECT pg_catalog.setval('public.source_type_id_seq', 14, true);


--
-- Name: company company_pkey; Type: CONSTRAINT; Schema: public; Owner: shane
--

ALTER TABLE ONLY public.company
    ADD CONSTRAINT company_pkey PRIMARY KEY (id);


--
-- Name: company_rating company_rating_pkey; Type: CONSTRAINT; Schema: public; Owner: shane
--

ALTER TABLE ONLY public.company_rating
    ADD CONSTRAINT company_rating_pkey PRIMARY KEY (id);


--
-- Name: company_urls company_urls_pkey; Type: CONSTRAINT; Schema: public; Owner: shane
--

ALTER TABLE ONLY public.company_urls
    ADD CONSTRAINT company_urls_pkey PRIMARY KEY (id);


--
-- Name: dataset_item dataset_item_pkey; Type: CONSTRAINT; Schema: public; Owner: shane
--

ALTER TABLE ONLY public.dataset_item
    ADD CONSTRAINT dataset_item_pkey PRIMARY KEY (id);


--
-- Name: dataset dataset_pkey; Type: CONSTRAINT; Schema: public; Owner: shane
--

ALTER TABLE ONLY public.dataset
    ADD CONSTRAINT dataset_pkey PRIMARY KEY (id);


--
-- Name: feed feed_pkey; Type: CONSTRAINT; Schema: public; Owner: shane
--

ALTER TABLE ONLY public.feed
    ADD CONSTRAINT feed_pkey PRIMARY KEY (id);


--
-- Name: feed feed_url; Type: CONSTRAINT; Schema: public; Owner: shane
--

ALTER TABLE ONLY public.feed
    ADD CONSTRAINT feed_url UNIQUE (url);


--
-- Name: garden garden_pkey; Type: CONSTRAINT; Schema: public; Owner: shane
--

ALTER TABLE ONLY public.garden
    ADD CONSTRAINT garden_pkey PRIMARY KEY (id);


--
-- Name: link link_pkey; Type: CONSTRAINT; Schema: public; Owner: shane
--

ALTER TABLE ONLY public.link
    ADD CONSTRAINT link_pkey PRIMARY KEY (id);


--
-- Name: news news_pkey; Type: CONSTRAINT; Schema: public; Owner: shane
--

ALTER TABLE ONLY public.news
    ADD CONSTRAINT news_pkey PRIMARY KEY (id);


--
-- Name: source_distribution newspaper_distribution_pkey; Type: CONSTRAINT; Schema: public; Owner: shane
--

ALTER TABLE ONLY public.source_distribution
    ADD CONSTRAINT newspaper_distribution_pkey PRIMARY KEY (id);


--
-- Name: page page_pkey; Type: CONSTRAINT; Schema: public; Owner: shane
--

ALTER TABLE ONLY public.page
    ADD CONSTRAINT page_pkey PRIMARY KEY (id);


--
-- Name: page_tag page_tag_pkey; Type: CONSTRAINT; Schema: public; Owner: shane
--

ALTER TABLE ONLY public.page_tag
    ADD CONSTRAINT page_tag_pkey PRIMARY KEY (id);


--
-- Name: person person_pkey; Type: CONSTRAINT; Schema: public; Owner: shane
--

ALTER TABLE ONLY public.person
    ADD CONSTRAINT person_pkey PRIMARY KEY (id);


--
-- Name: rating rating_pkey; Type: CONSTRAINT; Schema: public; Owner: shane
--

ALTER TABLE ONLY public.rating
    ADD CONSTRAINT rating_pkey PRIMARY KEY (id);


--
-- Name: scope scope_pkey; Type: CONSTRAINT; Schema: public; Owner: shane
--

ALTER TABLE ONLY public.scope
    ADD CONSTRAINT scope_pkey PRIMARY KEY (id);


--
-- Name: source_change source_change_pkey; Type: CONSTRAINT; Schema: public; Owner: shane
--

ALTER TABLE ONLY public.source_change
    ADD CONSTRAINT source_change_pkey PRIMARY KEY (id);


--
-- Name: source source_pk; Type: CONSTRAINT; Schema: public; Owner: shane
--

ALTER TABLE ONLY public.source
    ADD CONSTRAINT source_pk PRIMARY KEY (id);


--
-- Name: source_type source_type_pkey; Type: CONSTRAINT; Schema: public; Owner: shane
--

ALTER TABLE ONLY public.source_type
    ADD CONSTRAINT source_type_pkey PRIMARY KEY (id);


--
-- Name: source source_url; Type: CONSTRAINT; Schema: public; Owner: shane
--

ALTER TABLE ONLY public.source
    ADD CONSTRAINT source_url UNIQUE (url);


--
-- Name: tag tag_pkey; Type: CONSTRAINT; Schema: public; Owner: shane
--

ALTER TABLE ONLY public.tag
    ADD CONSTRAINT tag_pkey PRIMARY KEY (id);


--
-- Name: tool tool_pkey; Type: CONSTRAINT; Schema: public; Owner: shane
--

ALTER TABLE ONLY public.tool
    ADD CONSTRAINT tool_pkey PRIMARY KEY (id);


--
-- Name: page unique_garden_page_title_key; Type: CONSTRAINT; Schema: public; Owner: shane
--

ALTER TABLE ONLY public.page
    ADD CONSTRAINT unique_garden_page_title_key UNIQUE (garden_id, title);


--
-- Name: page unique_garden_slug_key; Type: CONSTRAINT; Schema: public; Owner: shane
--

ALTER TABLE ONLY public.page
    ADD CONSTRAINT unique_garden_slug_key UNIQUE (garden_id, slug);


--
-- Name: news unique_guid; Type: CONSTRAINT; Schema: public; Owner: shane
--

ALTER TABLE ONLY public.news
    ADD CONSTRAINT unique_guid UNIQUE (guid);


--
-- Name: company unique_name; Type: CONSTRAINT; Schema: public; Owner: shane
--

ALTER TABLE ONLY public.company
    ADD CONSTRAINT unique_name UNIQUE (name);


--
-- Name: source_type unique_name_idx; Type: CONSTRAINT; Schema: public; Owner: shane
--

ALTER TABLE ONLY public.source_type
    ADD CONSTRAINT unique_name_idx UNIQUE (name);


--
-- Name: tag unique_name_key; Type: CONSTRAINT; Schema: public; Owner: shane
--

ALTER TABLE ONLY public.tag
    ADD CONSTRAINT unique_name_key UNIQUE (name);


--
-- Name: company unique_ticker; Type: CONSTRAINT; Schema: public; Owner: shane
--

ALTER TABLE ONLY public.company
    ADD CONSTRAINT unique_ticker UNIQUE (ticker);


--
-- Name: garden unique_title_key; Type: CONSTRAINT; Schema: public; Owner: shane
--

ALTER TABLE ONLY public.garden
    ADD CONSTRAINT unique_title_key UNIQUE (title);


--
-- Name: company unique_url; Type: CONSTRAINT; Schema: public; Owner: shane
--

ALTER TABLE ONLY public.company
    ADD CONSTRAINT unique_url UNIQUE (url);


--
-- Name: urls urls_pkey; Type: CONSTRAINT; Schema: public; Owner: shane
--

ALTER TABLE ONLY public.urls
    ADD CONSTRAINT urls_pkey PRIMARY KEY (id);


--
-- Name: workspace workspace_pkey; Type: CONSTRAINT; Schema: public; Owner: shane
--

ALTER TABLE ONLY public.workspace
    ADD CONSTRAINT workspace_pkey PRIMARY KEY (id);


--
-- Name: company_rating company_rating_company_id_fk; Type: FK CONSTRAINT; Schema: public; Owner: shane
--

ALTER TABLE ONLY public.company_rating
    ADD CONSTRAINT company_rating_company_id_fk FOREIGN KEY (company_id) REFERENCES public.company(id);


--
-- Name: company_rating company_rating_rating_id_fk; Type: FK CONSTRAINT; Schema: public; Owner: shane
--

ALTER TABLE ONLY public.company_rating
    ADD CONSTRAINT company_rating_rating_id_fk FOREIGN KEY (rating_id) REFERENCES public.rating(id);


--
-- Name: page garden_fk; Type: FK CONSTRAINT; Schema: public; Owner: shane
--

ALTER TABLE ONLY public.page
    ADD CONSTRAINT garden_fk FOREIGN KEY (garden_id) REFERENCES public.garden(id);


--
-- Name: news news_feed_id_fk; Type: FK CONSTRAINT; Schema: public; Owner: shane
--

ALTER TABLE ONLY public.news
    ADD CONSTRAINT news_feed_id_fk FOREIGN KEY (feed_id) REFERENCES public.feed(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: page_tag page_fk; Type: FK CONSTRAINT; Schema: public; Owner: shane
--

ALTER TABLE ONLY public.page_tag
    ADD CONSTRAINT page_fk FOREIGN KEY (page_id) REFERENCES public.page(id);


--
-- Name: source_change source_change_source_id_fk; Type: FK CONSTRAINT; Schema: public; Owner: shane
--

ALTER TABLE ONLY public.source_change
    ADD CONSTRAINT source_change_source_id_fk FOREIGN KEY (source_id) REFERENCES public.source(id);


--
-- Name: source_distribution source_distribution_source_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: shane
--

ALTER TABLE ONLY public.source_distribution
    ADD CONSTRAINT source_distribution_source_id_fkey FOREIGN KEY (source_id) REFERENCES public.source(id);


--
-- Name: feed source_fk; Type: FK CONSTRAINT; Schema: public; Owner: shane
--

ALTER TABLE ONLY public.feed
    ADD CONSTRAINT source_fk FOREIGN KEY (source_id) REFERENCES public.source(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: link source_page_fk; Type: FK CONSTRAINT; Schema: public; Owner: shane
--

ALTER TABLE ONLY public.link
    ADD CONSTRAINT source_page_fk FOREIGN KEY (source_page_id) REFERENCES public.page(id);


--
-- Name: page_tag tag_fk; Type: FK CONSTRAINT; Schema: public; Owner: shane
--

ALTER TABLE ONLY public.page_tag
    ADD CONSTRAINT tag_fk FOREIGN KEY (tag_id) REFERENCES public.tag(id);


--
-- Name: link target_page_fk; Type: FK CONSTRAINT; Schema: public; Owner: shane
--

ALTER TABLE ONLY public.link
    ADD CONSTRAINT target_page_fk FOREIGN KEY (target_page_id) REFERENCES public.page(id);


--
-- Name: source type_fk; Type: FK CONSTRAINT; Schema: public; Owner: shane
--

ALTER TABLE ONLY public.source
    ADD CONSTRAINT type_fk FOREIGN KEY (type_id) REFERENCES public.source_type(id);


--
-- PostgreSQL database dump complete
--

-- Add migration script here
